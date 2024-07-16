use std::error::Error;

use dioxus::prelude::*;
use dioxus_components::prelude::*;

use crate::app::{ThemeColor, THEME};

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
    let selected_color = use_signal(|| THEME.read().primary.clone());
    let mut is_open = use_signal(|| true);

    rsx!(
        div {
            class: "absolute group right-0 bg-background border border-black w-fit flex justify-between transition-all",
            "data-open": is_open(),
            div {
                svg {
                    onclick: move |_| {
                        *is_open.write() = !is_open();
                    },
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 330 330",
                    class: "fill-foreground/70 size-4 transition-all duration-300 group-data-[open=false]:rotate-180",
                    path { d: "M165,0C74.019,0,0,74.019,0,165s74.019,165,165,165s165-74.019,165-165S255.981,0,165,0z M225.606,175.605  l-80,80.002C142.678,258.535,138.839,260,135,260s-7.678-1.464-10.606-4.394c-5.858-5.857-5.858-15.355,0-21.213l69.393-69.396  l-69.393-69.392c-5.858-5.857-5.858-15.355,0-21.213c5.857-5.858,15.355-5.858,21.213,0l80,79.998  c2.814,2.813,4.394,6.628,4.394,10.606C230,168.976,228.42,172.792,225.606,175.605z" }
                }
            }
            div { class: "w-fit group-data-[open=false]:hidden",
                Input {
                    r#type: "color",
                    value: hsl_to_hex(selected_color()),
                    oninput: move |event: FormEvent| {
                        if let Ok(color) = hex_to_hsl(&event.data().value()) {
                            match &*selected_color.read() {
                                ThemeColor::Primary(_) => {
                                    THEME.write().primary = ThemeColor::Primary(color);
                                }
                                ThemeColor::Secondary(_) => {
                                    THEME.write().secondary = ThemeColor::Secondary(color);
                                }
                                ThemeColor::Border(_) => {
                                    THEME.write().border = ThemeColor::Border(color);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                ColorSelector { selected_color, main_theme_color: THEME.read().primary.clone() }
                ColorSelector { selected_color, main_theme_color: THEME.read().secondary.clone() }
                ColorSelector { selected_color, main_theme_color: THEME.read().border.clone() }
            }
        }
    )
}

#[component]
fn ColorSelector(mut selected_color: Signal<ThemeColor>, main_theme_color: ThemeColor) -> Element {
    rsx!(
        div {
            role: "button",
            class: "rounded-full border-4 border-primary py-2 px-3 font-bold",
            class: if selected_color() == main_theme_color { "animate-spin" },
            onclick: move |_| {
                *selected_color.write() = main_theme_color.clone();
            },
            "P"
        }
    )
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

    Ok(HslColor {
        h: h as i64,
        s: s * 100.0,
        l: l * 100.0,
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
