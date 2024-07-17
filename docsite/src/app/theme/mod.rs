use std::error::Error;
use dioxus::prelude::*;
use dioxus_components::prelude::*;

use crate::app::{RadiusCss, ThemeColor, THEME};
use super::HslColor;

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
    let selected_color = use_signal(|| THEME.read().themes[0].primary.clone());
    let current_theme = THEME.read().current_theme;

    rsx!(
        div { class: "transition-all hidden group-data-[open=true]:block",
            div { class: "flex flex-col items-start pb-4 space-y-2",
                p { class: "text-sm font-medium",
                    "Selected: {THEME.read().themes[current_theme].name} {selected_color.read().to_string()}"
                }
                Input {
                    r#type: "color",
                    value: hsl_to_hex(selected_color()),
                    class: "cursor-pointer",
                    oninput: move |event: FormEvent| {
                        if let Ok(color) = hex_to_hsl(&event.data().value()) {
                            let current_theme = THEME.read().current_theme;
                            match *selected_color.read() {
                                ThemeColor::Background(_) => {
                                    THEME.write().themes[current_theme].background = ThemeColor::Background(
                                        color,
                                    );
                                }
                                ThemeColor::Foreground(_) => {
                                    THEME.write().themes[current_theme].foreground = ThemeColor::Foreground(
                                        color,
                                    );
                                }
                                ThemeColor::Primary(_) => {
                                    THEME.write().themes[current_theme].primary = ThemeColor::Primary(
                                        color,
                                    );
                                }
                                ThemeColor::PrimaryForeground(_) => {
                                    THEME.write().themes[current_theme].primary_foreground = ThemeColor::PrimaryForeground(
                                        color,
                                    );
                                }
                                ThemeColor::Secondary(_) => {
                                    THEME.write().themes[current_theme].secondary = ThemeColor::Secondary(
                                        color,
                                    );
                                }
                                ThemeColor::SecondaryForeground(_) => {
                                    THEME.write().themes[current_theme].secondary_foreground = ThemeColor::SecondaryForeground(
                                        color,
                                    );
                                }
                                ThemeColor::Accent(_) => {
                                    THEME.write().themes[current_theme].accent = ThemeColor::Accent(
                                        color,
                                    );
                                }
                                ThemeColor::AccentForeground(_) => {
                                    THEME.write().themes[current_theme].accent_foreground = ThemeColor::AccentForeground(
                                        color,
                                    );
                                }
                                ThemeColor::Muted(_) => {
                                    THEME.write().themes[current_theme].muted = ThemeColor::Muted(color);
                                }
                                ThemeColor::MutedForeground(_) => {
                                    THEME.write().themes[current_theme].muted_foreground = ThemeColor::MutedForeground(
                                        color,
                                    );
                                }
                                ThemeColor::Destructive(_) => {
                                    THEME.write().themes[current_theme].destructive = ThemeColor::Destructive(
                                        color,
                                    );
                                }
                                ThemeColor::DestructiveForeground(_) => {
                                    THEME.write().themes[current_theme].destructive_foreground = ThemeColor::DestructiveForeground(
                                        color,
                                    );
                                }
                                ThemeColor::Success(_) => {
                                    THEME.write().themes[current_theme].success = ThemeColor::Success(
                                        color,
                                    );
                                }
                                ThemeColor::SuccessForeground(_) => {
                                    THEME.write().themes[current_theme].success_foreground = ThemeColor::SuccessForeground(
                                        color,
                                    );
                                }
                                ThemeColor::Border(_) => {
                                    THEME.write().themes[current_theme].border = ThemeColor::Border(
                                        color,
                                    );
                                }
                                ThemeColor::Input(_) => {
                                    THEME.write().themes[current_theme].input = ThemeColor::Input(color);
                                }
                                ThemeColor::Ring(_) => {
                                    THEME.write().themes[current_theme].ring = ThemeColor::Ring(color);
                                }
                            }
                        }
                    }
                }
                Scrollable { orientation: Orientation::Vertical, class: "h-64 space-y-1",
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].background.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].foreground.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].primary.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].primary_foreground.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].secondary.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].secondary_foreground.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].accent.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].accent_foreground.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].muted.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].muted_foreground.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].destructive.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].destructive_foreground.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].success.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].success_foreground.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].border.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].input.clone()
                    }
                    ColorSelector {
                        selected_color,
                        main_theme_color: THEME.read().themes[THEME.read().current_theme].ring.clone()
                    }
                }
                RadiusSelector {}
                ButtonExport {}
            }
        }
    )
}

#[component]
fn ColorSelector(mut selected_color: Signal<ThemeColor>, main_theme_color: ThemeColor) -> Element {
    // The dynamic string in class only works because we are compiling all of tailwind classes
    rsx!(
        div {
            role: "button",
            class: "rounded-global-radius border-2 border-{main_theme_color.to_string()} py-2 px-3 font-bold text-sm",
            class: if selected_color() == main_theme_color { "bg-accent text-accent-foreground" },
            onclick: move |_| {
                *selected_color.write() = main_theme_color.clone();
            },
            "{main_theme_color.to_string()}"
        }
    )
}

#[component]
fn RadiusSelector() -> Element {
    let current_theme = THEME.read().current_theme;
    
    rsx!(
        div { id: "radius-selector", class: "w-full",
            p { class: "text-sm font-medium", "Radius" }
            Input {
                size: Size::Sm,
                r#type: "text",
                value: THEME.read().themes[current_theme].radius.to_style(),
                oninput: move |event: FormEvent| {
                    let value = event.data().value();
                    THEME.write().themes[current_theme].radius = RadiusCss(value);
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
    let current_theme = THEME.read().current_theme;
    let current_theme_name = THEME.read().themes[current_theme].name.clone();


    rsx!(
        div {
            class: "cursor-pointer p-1 rounded-global-radius hover:bg-foreground/40 active:bg-foreground/60",
            onclick: move |_| {
                if current_theme == 0 {
                    THEME.write().current_theme = 1
                } else {
                    THEME.write().current_theme = 0
                }
            },
            if current_theme_name == "light" {
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
    rsx!{
        Scrollable { orientation: Orientation::Vertical, class: "max-h-80 border-none bg-foreground",
            pre { class: "bg-foreground text-background pl-4 pr-12 py-2 rounded-global-radius",
                code { class: "text-sm font-mono", {THEME.read().export_themes()} }
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

    Ok(HslColor {
        h: h as i64,
        s,
        l,
    })
}

fn hsl_to_hex(color: ThemeColor) -> String {
    match color {
        ThemeColor::Primary(HslColor { h, s, l }) => hsl_to_hex_string(h, s, l),
        ThemeColor::Secondary(HslColor { h, s, l }) => hsl_to_hex_string(h, s, l),
        ThemeColor::Border(HslColor { h, s, l }) => hsl_to_hex_string(h, s, l),
        _ => "#000000".to_string(),
    }
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
