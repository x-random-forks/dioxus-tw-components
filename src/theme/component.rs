use crate::{prelude::*, theme::*};
use dioxus::prelude::*;

#[component]
pub fn ThemePicker() -> Element {
    let is_open = use_signal(|| false);

    rsx!(
        div {
            class: "group fixed right-0 top-1/4 z-10 bg-background rounded-global-radius border border-border space-x-2 p-2 flex justify-between overflow-hidden",
            "data-open": is_open(),
            MiniPicker { is_open }
            ColorPicker {}
        }
    )
}

#[component]
fn ColorPicker() -> Element {
    let selected_color = use_signal(|| String::from("primary"));

    let mut theme_manager = use_context::<Signal<ThemeManager>>();

    let current_theme = theme_manager.read().current_theme;

    let oninput = move |event: FormEvent| {
        // TODO Very ugly but works

        // Convert the event value to an HslColor struct
        let Ok(hsl_color) = HslColor::try_new_from_hex(&event.data().value()) else {
            return;
        };

        // We check if the selected color is the default one, or if it is a foreground color
        let select_color = if *selected_color.read() == "foreground" {
            "background".to_string()
        } else if selected_color.read().contains("foreground") {
            selected_color.read().replace("-foreground", "")
        } else {
            selected_color.read().to_string()
        };

        // Get the current selected color in the theme manager (as mut ref)
        if let Some(color_choice) = theme_manager.write().themes[current_theme]
            .colors
            .get_mut(&select_color)
        {
            match color_choice {
                ColorChoice::Simple(color) => {
                    *color = hsl_color;
                }
                ColorChoice::Duo(color_bg, color_fg) => {
                    if select_color == "background" && *selected_color.read() == "foreground" {
                        *color_fg = hsl_color;
                        return;
                    }

                    if selected_color.read().contains("foreground") {
                        *color_fg = hsl_color;
                    } else {
                        *color_bg = hsl_color;
                    }
                }
            }
        }
    };

    rsx!(
        div { class: "transition-all hidden group-data-[open=true]:block border-l pl-4",
            div { class: "flex flex-col items-start pb-4 space-y-2",
                Input {
                    role: "button",
                    id: "color-picker-input",
                    r#type: "color",
                    oninput
                }
                p { class: "text-sm font-medium",
                    "Selected: {theme_manager.read().themes[current_theme].name} {selected_color.read().to_string()}"
                }
                for (str , color) in theme_manager.read().themes[current_theme].colors.iter() {
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

            let is_selected = if &*color_str.read() == "background" {
                selected_color() == "foreground"
            } else {
                selected_color() == format!("{}-foreground", color_str)
            };

            rsx!(
                ToggleDiv {
                    is_selected,
                    onclick: move |_| {
                        if &*color_str.read() == "background" {
                            log::debug!("there");
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
                        selected_color.write().clone_from(&*color_str.read());
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
    let onclick = move |event| {
        onclick.call(event);

        // This is to trigger the click event on the color picker input
        // Will only work with web
        spawn(async move {
            let _eval = eval(
                r#"
                const input = document.getElementById("color-picker-input");
                if (input != null) {
                    input.click();
                }
                "#,
            );
        });
    };

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
    let mut theme_manager = use_context::<Signal<ThemeManager>>();

    let current_theme = theme_manager.read().current_theme;

    rsx!(
        div { id: "radius-selector", class: "w-full",
            p { class: "text-sm font-medium", "Radius" }
            Input {
                size: Size::Sm,
                r#type: "text",
                value: theme_manager.read().themes[current_theme].radius.to_style(),
                oninput: move |event: FormEvent| {
                    let value = event.data().value();
                    theme_manager.write().themes[current_theme].radius = RadiusCss(value);
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
    let mut theme_manager = use_context::<Signal<ThemeManager>>();

    let current_theme = theme_manager.read().current_theme;
    let current_theme_name = theme_manager.read().themes[current_theme].name.clone();

    rsx!(
        div {
            class: "cursor-pointer p-1 rounded-global-radius hover:bg-foreground/40 active:bg-foreground/60",
            onclick: move |_| {
                if current_theme == 0 {
                    theme_manager.write().current_theme = 1
                } else {
                    theme_manager.write().current_theme = 0
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

#[component]
fn ThemeExport() -> Element {
    let theme_manager = use_context::<Signal<ThemeManager>>();

    rsx! {
        Scrollable { orientation: Orientation::Vertical, class: "max-h-80 border-none bg-foreground",
            pre { class: "bg-foreground text-background pl-4 pr-12 py-2 rounded-global-radius",
                code { class: "text-sm", "{theme_manager.read().export_to_css()}" }
            }
        }
    }
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
