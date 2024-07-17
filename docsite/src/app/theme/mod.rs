use dioxus::prelude::*;
use dioxus_components::prelude::*;
use dioxus_elements::style;
use std::{collections::HashMap, error::Error};

pub static THEME_MANAGER: GlobalSignal<ThemeManager> = Signal::global(|| ThemeManager::default());

pub fn ThemePage() -> Element {
    rsx!(
        div {
            "THEMEME"
            ThemePicker {}
        }
    )
}

pub fn ThemePicker() -> Element {
    let is_open = use_signal(|| true);

    rsx!(
        div {
            class: "absolute group right-0 z-10 bg-background rounded-global-radius border border-border space-x-2 p-2 flex justify-between overflow-hidden",
            "data-open": is_open(),
            MiniPicker { is_open }
            ColorPicker {}
        }
    )
}

#[component]
fn ColorPicker() -> Element {
    let selected_color = use_signal(|| String::from("primary"));

    let curr_theme = THEME_MANAGER.read().current_theme;

    let oninput = move |event: FormEvent| {
        // Convert the event value to an HslColor struct
        let Ok(hsl_color) = HslColor::try_new_from_hex(&event.data().value()) else {
            return;
        };

        // Get the current selected color in the theme manager (as mut ref)
        if let Some(color_choice) = THEME_MANAGER.write().themes[curr_theme]
            .colors
            .get_mut(&selected_color())
        {
            match color_choice {
                ColorChoice::Simple(color) => {
                    *color = hsl_color;
                }
                ColorChoice::Duo(color_bg, color_fg) => {
                    if selected_color().contains("foreground") {
                        *color_fg = hsl_color;
                    } else {
                        *color_bg = hsl_color;
                    }
                }
            }
        }
    };

    let mut rect_input = use_signal(|| None);

    rsx!(
        div { class: "transition-all hidden group-data-[open=true]:block border-l pl-4",
            div { class: "flex flex-col items-start pb-4 space-y-2",
                div {
                    onclick: move |event: MouseEvent| {
                        log::debug!("CLICK");
                    },
                    "CLICK"
                }
                input {
                    onmounted: move |event: MountedEvent| async move {
                        let rect = event.get_client_rect().await;
                        *rect_input.write() = Some(rect);
                    },
                    id: "color-picker",
                    r#type: "color",
                    oninput
                }
                p { class: "text-sm font-medium",
                    "Selected: {THEME_MANAGER.read().themes[curr_theme].name} {selected_color.read().to_string()}"
                }
                for (str , color) in THEME_MANAGER.read().themes[curr_theme].colors.iter() {
                    ColorSelector { color_str: str, color: color.clone(), selected_color }
                }
                RadiusSelector {}
                ButtonExport {}
            }
        }
    )
}

#[component]
fn ColorSelector(
    color_str: ReadOnlySignal<String>,
    color: ColorChoice,
    mut selected_color: Signal<String>,
) -> Element {
    let content: Element = match color {
        ColorChoice::Simple(_) => {
            rsx!(
                ToggleDiv {
                    is_selected: selected_color() == format!("{}-foreground", color_str),
                    onclick: move |_| {
                        *selected_color.write() = format!("{}-foreground", color_str);
                    },
                    SvgEyedropper {}
                }
                div { class: "bg-{color_str} grow flex items-center justify-center",
                    p {
                        color: "white",
                        text_shadow: "-1px -1px 0 #000, 1px -1px 0 #000, -1px 1px 0 #000, 1px 1px 0 #000",
                        "{color_str}"
                    }
                }
            )
        }
        ColorChoice::Duo(_, _) => {
            let bg = if &*color_str.read() == "background" {
                "bg-background".to_string()
            } else {
                format!("bg-{}", color_str)
            };

            let text = if &*color_str.read() == "background" {
                "text-foreground".to_string()
            } else {
                format!("text-{}-foreground", color_str)
            };

            rsx!(
                ToggleDiv {
                    is_selected: selected_color() == format!("{}-foreground", color_str),
                    onclick: move |_| {
                        if &*color_str.read() == "background" {
                            *selected_color.write() = "foreground".to_string();
                        } else {
                            *selected_color.write() = format!("{}-foreground", color_str);
                        }
                    },
                    SvgForeground {}
                }
                p { class: "{bg} {text} text-center text-sm font-bold grow flex items-center justify-center",
                    "{color_str}"
                }
                ToggleDiv {
                    is_selected: *selected_color.read() == *color_str.read(),
                    onclick: move |_| {
                        *selected_color.write() = color_str.read().clone();
                    },
                    SvgBackground {}
                }
            )
        }
    };

    rsx!(
        div { class: "w-full rounded-global-radius p-1 space-x-1 flex bg-backgroud text-foreground font-bold text-sm border border-border",
            { content }
        }
    )
}

#[component]
fn ToggleDiv(is_selected: bool, onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    let onclick = move |event| onclick.call(event);

    rsx!(
        div {
            role: "button",
            class: "group rounded-global-radius p-1 transition-all data-[selected=true]:bg-accent active:bg-foreground/45",
            "data-selected": is_selected,
            onclick,
            {children}
        }
    )
}

#[component]
fn RadiusSelector() -> Element {
    let current_theme = THEME_MANAGER.read().current_theme;

    rsx!(
        div { id: "radius-selector", class: "w-full",
            p { class: "text-sm font-medium", "Radius" }
            Input {
                size: Size::Sm,
                r#type: "text",
                value: THEME_MANAGER.read().themes[current_theme].radius.to_style(),
                oninput: move |event: FormEvent| {
                    let value = event.data().value();
                    THEME_MANAGER.write().themes[current_theme].radius = RadiusCss(value);
                }
            }
        }
    )
}

#[component]
fn MiniPicker(mut is_open: Signal<bool>) -> Element {
    rsx!(
        div { class: "flex flex-col items-center space-y-2",
            svg {
                id: "open-theme-picker",
                onclick: move |_| {
                    *is_open.write() = !is_open();
                },
                role: "button",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 330 330",
                class: "fill-foreground/70 size-4 transition-all duration-300 group-data-[open=false]:rotate-180",
                path { d: "M165,0C74.019,0,0,74.019,0,165s74.019,165,165,165s165-74.019,165-165S255.981,0,165,0z M225.606,175.605  l-80,80.002C142.678,258.535,138.839,260,135,260s-7.678-1.464-10.606-4.394c-5.858-5.857-5.858-15.355,0-21.213l69.393-69.396  l-69.393-69.392c-5.858-5.857-5.858-15.355,0-21.213c5.857-5.858,15.355-5.858,21.213,0l80,79.998  c2.814,2.813,4.394,6.628,4.394,10.606C230,168.976,228.42,172.792,225.606,175.605z" }
            }
            ThemeSwitcher { is_open }
        }
    )
}

#[component]
fn ThemeSwitcher(is_open: ReadOnlySignal<bool>) -> Element {
    let current_theme = THEME_MANAGER.read().current_theme;
    let current_theme_name = THEME_MANAGER.read().themes[current_theme].name.clone();

    rsx!(
        div {
            class: "cursor-pointer p-1 rounded-global-radius hover:bg-foreground/40 active:bg-foreground/60",
            onclick: move |_| {
                if current_theme == 0 {
                    THEME_MANAGER.write().current_theme = 1
                } else {
                    THEME_MANAGER.write().current_theme = 0
                }
            },
            if current_theme_name == "root" {
                svg {
                    view_box: "0 0 24 24",
                    width: 24,
                    height: 24,
                    stroke_width: 2,
                    fill: "none",
                    class: "stroke-foreground",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: 12, cy: 12, r: 5 }
                    line {
                        x1: 12,
                        y1: 1,
                        x2: 12,
                        y2: 3
                    }
                    line {
                        x1: 12,
                        y1: 21,
                        x2: 12,
                        y2: 23
                    }
                    line {
                        x1: 4.22,
                        y1: 4.22,
                        x2: 5.64,
                        y2: 5.64
                    }
                    line {
                        x1: 18.36,
                        y1: 18.36,
                        x2: 19.78,
                        y2: 19.78
                    }
                    line {
                        x1: 1,
                        y1: 12,
                        x2: 3,
                        y2: 12
                    }
                    line {
                        x1: 21,
                        y1: 12,
                        x2: 23,
                        y2: 12
                    }
                    line {
                        x1: 4.22,
                        y1: 19.78,
                        x2: 5.64,
                        y2: 18.36
                    }
                    line {
                        x1: 18.36,
                        y1: 5.64,
                        x2: 19.78,
                        y2: 4.22
                    }
                }
            } else {
                svg {
                    view_box: "0 0 24 24",
                    width: 24,
                    height: 24,
                    class: "stroke-foreground",
                    stroke_width: 2,
                    fill: "none",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
                }
            }
        }
    )
}

#[component]
fn ButtonExport() -> Element {
    rsx!(
        Modal { 
            ModalTrigger { class: "w-full text-center", "Export Theme" }
            ModalBackground {}
            ModalContent { 
                ModalClose {}
                h6 { class: "h6", "Theme" }
                p { class: "text-sm font-medium text-foreground/50 pb-4",
                    "Copy and paste this in your project's CSS file."
                }
                ThemeExport {}
            }
        }
    )
}

pub fn ThemeExport() -> Element {
    rsx! {
        Scrollable { orientation: Orientation::Vertical, class: "max-h-80 border-none bg-foreground",
            pre { class: "bg-foreground text-background pl-4 pr-12 py-2 rounded-global-radius",
                code { class: "text-sm", "{THEME_MANAGER.read().export_to_css()}" }
            }
        }
    }
}

fn hex_to_hsl(hex: &str) -> Result<HslColor, Box<dyn Error>> {
    // Remove the '#' if present
    let hex = hex.trim_start_matches('#');

    // Parse the hex string
    let rgb: u32 = u32::from_str_radix(hex, 16)?;

    // Extract RGB components
    let r = ((rgb >> 16) & 255) as f64 / 255.0;
    let g = ((rgb >> 8) & 255) as f64 / 255.0;
    let b = (rgb & 255) as f64 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let diff = max - min;

    // Calculate HSL components
    let h = if diff == 0.0 {
        0.0
    } else if max == r {
        (g - b) / diff % 6.0
    } else if max == g {
        (b - r) / diff + 2.0
    } else {
        (r - g) / diff + 4.0
    };

    let h = h * 60.0;
    let l = (max + min) / 2.0;
    let s = if diff == 0.0 {
        0.0
    } else {
        diff / (1.0 - (2.0 * l - 1.0).abs())
    };

    let s_trunc = format!("{:.3}", s * 100.0);
    let s = s_trunc.parse::<f64>()?;

    let l_trunc = format!("{:.3}", l * 100.0);
    let l = l_trunc.parse::<f64>()?;

    Ok(HslColor { h: h as i64, s, l })
}

fn hsl_to_hex_string(h: i64, s: f64, l: f64) -> String {
    let h = h as f64 / 360.0;
    let s = s / 100.0;
    let l = l / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0).round() as u8;
    let g = ((g + m) * 255.0).round() as u8;
    let b = ((b + m) * 255.0).round() as u8;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn SvgForeground() -> Element {
    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 512 512",
            class: "size-6 fill-foreground group-data-[selected=true]:fill-accent-foreground",
            path { d: "M395.636,232.727c12.87,0,23.273-10.426,23.273-23.273c0-12.846-10.403-23.273-23.273-23.273h-23.273     c-12.87,0-23.273,10.426-23.273,23.273c0,12.847,10.403,23.273,23.273,23.273H395.636z" }
            path { d: "M232.727,465.455h-23.273c-12.87,0-23.273,10.426-23.273,23.273S196.585,512,209.455,512h23.273     c12.87,0,23.273-10.426,23.273-23.273S245.597,465.455,232.727,465.455z" }
            path { d: "M209.455,372.364c-12.87,0-23.273,10.426-23.273,23.273v23.273c0,12.847,10.403,23.273,23.273,23.273     c12.87,0,23.273-10.426,23.273-23.273v-23.273C232.727,382.79,222.324,372.364,209.455,372.364z" }
            path { d: "M209.455,349.091c12.87,0,23.273-10.426,23.273-23.273h46.545h23.273c12.87,0,23.273-10.426,23.273-23.273v-69.818     v-23.273V23.273C325.818,10.426,315.415,0,302.545,0H23.273C10.403,0,0,10.426,0,23.273v279.273     c0,12.847,10.403,23.273,23.273,23.273h162.909C186.182,338.665,196.585,349.091,209.455,349.091z" }
            path { d: "M325.818,465.455h-23.273c-12.87,0-23.273,10.426-23.273,23.273S289.676,512,302.545,512h23.273     c12.87,0,23.273-10.426,23.273-23.273S338.688,465.455,325.818,465.455z" }
            path { d: "M488.727,256c-12.87,0-23.273,10.426-23.273,23.273v23.273c0,12.847,10.403,23.273,23.273,23.273     S512,315.392,512,302.545v-23.273C512,266.426,501.597,256,488.727,256z" }
            path { d: "M488.727,349.091c-12.87,0-23.273,10.426-23.273,23.273v23.273c0,12.847,10.403,23.273,23.273,23.273     S512,408.483,512,395.636v-23.273C512,359.517,501.597,349.091,488.727,349.091z" }
            path { d: "M488.727,442.182c-12.87,0-23.273,10.426-23.273,23.273v23.273c0,12.847,10.403,23.273,23.273,23.273     S512,501.574,512,488.727v-23.273C512,452.608,501.597,442.182,488.727,442.182z" }
            path { d: "M418.909,465.455h-23.273c-12.87,0-23.273,10.426-23.273,23.273S382.767,512,395.636,512h23.273     c12.87,0,23.273-10.426,23.273-23.273S431.779,465.455,418.909,465.455z" }
            path { d: "M488.727,232.727c12.87,0,23.273-10.426,23.273-23.273c0-12.846-10.403-23.273-23.273-23.273h-23.273     c-12.87,0-23.273,10.426-23.273,23.273c0,12.847,10.403,23.273,23.273,23.273H488.727z" }
        }
    )
}

fn SvgBackground() -> Element {
    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 512 512",
            class: "size-6 fill-foreground rotate-180 group-data-[selected=true]:fill-accent-foreground",
            path { d: "M384,469.333h-21.333c-11.797,0-21.333,9.557-21.333,21.333S350.869,512,362.667,512H384     c11.797,0,21.333-9.557,21.333-21.333S395.797,469.333,384,469.333z" }
            path { d: "M149.333,426.667c11.797,0,21.333-9.557,21.333-21.333V384c0-11.776-9.536-21.333-21.333-21.333     C137.536,362.667,128,372.224,128,384v21.333C128,417.109,137.536,426.667,149.333,426.667z" }
            path { d: "M170.667,469.333h-21.333c-11.797,0-21.333,9.557-21.333,21.333S137.536,512,149.333,512h21.333     c11.797,0,21.333-9.557,21.333-21.333S182.464,469.333,170.667,469.333z" }
            path { d: "M277.333,469.333H256c-11.797,0-21.333,9.557-21.333,21.333S244.203,512,256,512h21.333     c11.797,0,21.333-9.557,21.333-21.333S289.131,469.333,277.333,469.333z" }
            path { d: "M362.667,149.333c0-11.776-9.536-21.333-21.333-21.333H320c-11.797,0-21.333,9.557-21.333,21.333     s9.536,21.333,21.333,21.333h21.333C353.131,170.667,362.667,161.109,362.667,149.333z" }
            path { d: "M490.667,384c-11.797,0-21.333,9.557-21.333,21.333v21.333c0,11.776,9.536,21.333,21.333,21.333S512,438.443,512,426.667     v-21.333C512,393.557,502.464,384,490.667,384z" }
            path { d: "M490.667,277.333c-11.797,0-21.333,9.557-21.333,21.333V320c0,11.776,9.536,21.333,21.333,21.333S512,331.776,512,320     v-21.333C512,286.891,502.464,277.333,490.667,277.333z" }
            path { d: "M490.667,170.667c-11.797,0-21.333,9.557-21.333,21.333v21.333c0,11.776,9.536,21.333,21.333,21.333     S512,225.109,512,213.333V192C512,180.224,502.464,170.667,490.667,170.667z" }
            path { d: "M469.333,149.333c0-11.776-9.536-21.333-21.333-21.333h-21.333c-11.797,0-21.333,9.557-21.333,21.333     s9.536,21.333,21.333,21.333H448C459.797,170.667,469.333,161.109,469.333,149.333z" }
            path { d: "M490.667,469.333h-21.333c-11.797,0-21.333,9.557-21.333,21.333S457.536,512,469.333,512h21.333     c11.797,0,21.333-9.557,21.333-21.333S502.464,469.333,490.667,469.333z" }
            path { d: "M277.333,0h-256C9.536,0,0,9.557,0,21.333v256c0,11.776,9.536,21.333,21.333,21.333H128     c0,11.776,9.536,21.333,21.333,21.333c11.797,0,21.333-9.557,21.333-21.333v-21.333V192v-21.333h42.667h21.333h42.667     c11.797,0,21.333-9.557,21.333-21.333v-128C298.667,9.557,289.131,0,277.333,0z" }
        }
    )
}

fn SvgEyedropper() -> Element {
    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            class: "size-6 fill-foreground group-data-[selected=true]:fill-accent-foreground",
            path { d: "M20.453,8.936a1.961,1.961,0,0,1-2.772,2.772L12.292,6.319a1.961,1.961,0,0,1,2.772-2.772l.673.674a2.859,2.859,0,1,1,4.042,4.041ZM5.286,15.48A2.264,2.264,0,0,0,4.7,16.513,2.257,2.257,0,0,0,3.67,17.1,2.286,2.286,0,0,0,6.9,20.33,2.257,2.257,0,0,0,7.487,19.3a2.264,2.264,0,0,0,1.033-.584l6.67-6.67L11.956,8.81Z" }
        }
    )
}

pub trait ToStyle {
    fn to_style(&self) -> String;
}

pub trait ExportToCss {
    fn export_to_css(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ThemeManager {
    themes: Vec<Theme>,
    current_theme: usize,
}

impl std::default::Default for ThemeManager {
    fn default() -> Self {
        Self {
            themes: vec![Theme::default(), Theme::dark()],
            current_theme: 0,
        }
    }
}

impl ToStyle for ThemeManager {
    fn to_style(&self) -> String {
        self.themes[self.current_theme].to_style()
    }
}

impl ExportToCss for ThemeManager {
    fn export_to_css(&self) -> String {
        let mut css = String::from("@layer base {\n\n");

        for theme in self.themes.iter() {
            css.push_str(&theme.export_to_css());
            css.push_str("\n");
        }

        css.push_str("}");
        css
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    colors: HashMap<String, ColorChoice>,
    radius: RadiusCss,
}

impl ToStyle for Theme {
    fn to_style(&self) -> String {
        let mut style = String::new();

        for (key, color_choice) in self.colors.iter() {
            match color_choice {
                ColorChoice::Simple(color) => {
                    style.push_str(&format!("--{}: {};", key, color.to_style()));
                }
                ColorChoice::Duo(background, foreground) => {
                    style.push_str(&format!("--{}: {};", key, background.to_style()));
                    if key != "background" {
                        style.push_str(&format!(
                            "--{}-foreground: {};",
                            key,
                            foreground.to_style()
                        ));
                    } else {
                        style.push_str(&format!("--foreground: {};", foreground.to_style()));
                    }
                }
            }
        }

        style.push_str(&format!("--global-radius: {};\n", self.radius.to_style()));

        style
    }
}

impl ExportToCss for Theme {
    fn export_to_css(&self) -> String {
        let first_char = if self.name == "root" { ':' } else { '.' };

        let mut css = format!(" {}{} {{\n", first_char, self.name);

        for (key, color_choice) in self.colors.iter() {
            match color_choice {
                ColorChoice::Simple(color) => {
                    css.push_str(&format!("  --{}: {};\n", key, color.to_style()));
                }
                ColorChoice::Duo(background, foreground) => {
                    css.push_str(&format!("  --{}: {};\n", key, background.to_style()));
                    if key != "background" {
                        css.push_str(&format!(
                            "  --{}-foreground: {};\n",
                            key,
                            foreground.to_style()
                        ));
                    } else {
                        css.push_str(&format!("  --foreground: {};\n", foreground.to_style()));
                    }
                }
            }
        }

        css.push_str(&format!("  --global-radius: {};\n", self.radius.to_style()));

        css.push_str(" }\n");

        css
    }
}

impl std::default::Default for Theme {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert(
            "background".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
                HslColor {
                    h: 240,
                    s: 0.1,
                    l: 3.9,
                },
            ),
        );

        colors.insert(
            "primary".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 292,
                    s: 1.0,
                    l: 30.0,
                },
                HslColor {
                    h: 355,
                    s: 1.0,
                    l: 97.3,
                },
            ),
        );

        colors.insert(
            "secondary".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 123,
                    s: 1.0,
                    l: 22.0,
                },
                HslColor {
                    h: 123,
                    s: 1.0,
                    l: 70.0,
                },
            ),
        );

        colors.insert(
            "accent".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 85.6,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 17.4,
                },
            ),
        );

        colors.insert(
            "muted".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 240,
                    s: 0.048,
                    l: 95.9,
                },
                HslColor {
                    h: 240,
                    s: 0.038,
                    l: 46.1,
                },
            ),
        );

        colors.insert(
            "destructive".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 0,
                    s: 0.842,
                    l: 60.2,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 98.0,
                },
            ),
        );

        colors.insert(
            "success".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 142,
                    s: 0.7,
                    l: 45.0,
                },
                HslColor {
                    h: 120,
                    s: 1.0,
                    l: 97.3,
                },
            ),
        );

        colors.insert(
            "border".to_string(),
            ColorChoice::Simple(HslColor {
                h: 240,
                s: 0.059,
                l: 90.0,
            }),
        );

        colors.insert(
            "input".to_string(),
            ColorChoice::Simple(HslColor {
                h: 240,
                s: 0.059,
                l: 90.0,
            }),
        );

        colors.insert(
            "ring".to_string(),
            ColorChoice::Simple(HslColor {
                h: 240,
                s: 0.1,
                l: 3.9,
            }),
        );

        Self {
            name: "root".to_string(),
            colors,
            radius: RadiusCss("10px".to_string()),
        }
    }
}

impl Theme {
    fn dark() -> Self {
        let mut colors = HashMap::new();
        colors.insert(
            "background".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 240,
                    s: 0.1,
                    l: 3.9,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 100.0,
                },
            ),
        );

        colors.insert(
            "primary".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 355,
                    s: 1.0,
                    l: 97.3,
                },
                HslColor {
                    h: 292,
                    s: 1.0,
                    l: 30.0,
                },
            ),
        );

        colors.insert(
            "secondary".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 123,
                    s: 1.0,
                    l: 70.0,
                },
                HslColor {
                    h: 123,
                    s: 1.0,
                    l: 22.0,
                },
            ),
        );

        colors.insert(
            "accent".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 17.4,
                },
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 85.6,
                },
            ),
        );

        colors.insert(
            "muted".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 240,
                    s: 0.038,
                    l: 46.1,
                },
                HslColor {
                    h: 240,
                    s: 0.048,
                    l: 95.9,
                },
            ),
        );

        colors.insert(
            "destructive".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 0,
                    s: 0.0,
                    l: 98.0,
                },
                HslColor {
                    h: 0,
                    s: 0.842,
                    l: 60.2,
                },
            ),
        );

        colors.insert(
            "success".to_string(),
            ColorChoice::Duo(
                HslColor {
                    h: 120,
                    s: 1.0,
                    l: 97.3,
                },
                HslColor {
                    h: 142,
                    s: 0.7,
                    l: 45.0,
                },
            ),
        );

        colors.insert(
            "border".to_string(),
            ColorChoice::Simple(HslColor {
                h: 240,
                s: 0.059,
                l: 90.0,
            }),
        );

        colors.insert(
            "input".to_string(),
            ColorChoice::Simple(HslColor {
                h: 240,
                s: 0.059,
                l: 90.0,
            }),
        );

        colors.insert(
            "ring".to_string(),
            ColorChoice::Simple(HslColor {
                h: 240,
                s: 0.1,
                l: 3.9,
            }),
        );

        Self {
            name: "dark".to_string(),
            colors,
            radius: RadiusCss("0px".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorChoice {
    Simple(HslColor),
    Duo(HslColor, HslColor),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HslColor {
    h: i64,
    s: f64,
    l: f64,
}

impl ToStyle for HslColor {
    fn to_style(&self) -> String {
        format!("{}deg {}% {}%", self.h, self.s, self.l)
    }
}

impl HslColor {
    pub fn try_new_from_hex(hex: &str) -> Result<Self, Box<dyn Error>> {
        let hex = hex.trim_start_matches('#');

        // Parse the hex string
        let rgb: u32 = u32::from_str_radix(hex, 16)?;

        // Extract RGB components
        let r = ((rgb >> 16) & 255) as f64 / 255.0;
        let g = ((rgb >> 8) & 255) as f64 / 255.0;
        let b = (rgb & 255) as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let diff = max - min;

        // Calculate HSL components
        let h = if diff == 0.0 {
            0.0
        } else if max == r {
            (g - b) / diff % 6.0
        } else if max == g {
            (b - r) / diff + 2.0
        } else {
            (r - g) / diff + 4.0
        };

        let h = h * 60.0;
        let l = (max + min) / 2.0;
        let s = if diff == 0.0 {
            0.0
        } else {
            diff / (1.0 - (2.0 * l - 1.0).abs())
        };

        let s_trunc = format!("{:.3}", s * 100.0);
        let s = s_trunc.parse::<f64>()?;

        let l_trunc = format!("{:.3}", l * 100.0);
        let l = l_trunc.parse::<f64>()?;

        Ok(Self { h: h as i64, s, l })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadiusCss(String);

impl RadiusCss {
    pub fn to_style(&self) -> String {
        self.0.clone()
    }
}
