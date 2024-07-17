pub mod components;
pub mod doctrait;
pub mod home;
pub mod layout;
pub mod router;
pub mod theme;

use dioxus::prelude::*;
use dioxus_components::molecules::Toaster;

use crate::app::router::Route;

static THEME: GlobalSignal<ThemePicker> = Signal::global(|| ThemePicker::default());

pub fn App() -> Element {
    rsx!(
        Toaster { 
            div {
                class: "relative bg-background text-foreground",
                style: THEME.read().to_style(),
                Router::<Route> {}
            }
        }
    )
}

#[derive(Debug, Clone)]
pub struct ThemePicker {
    themes: Vec<Theme>,
    current_theme: usize,
}

impl std::default::Default for ThemePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemePicker {
    pub fn new() -> Self {
        Self {
            themes: vec![Theme::default(), Theme::dark_default()],
            current_theme: 0,
        }
    }

    pub fn to_style(&self) -> String {
        self.themes[self.current_theme].to_style()
    }

    // Very dirty way for now but should work
    fn export_themes(&self) -> String {
        let mut themes = String::from("@layer base {\n");
        for (i, theme) in self.themes.iter().enumerate() {
            let name = if theme.name == "light" {
                ":root".to_string()
            } else {
                format!(".{}", theme.name)
            };

            themes.push_str(&format!(" {} {{\n", name));
            themes.push_str(&format!("{}\n }}\n", theme.to_style()));
            if i < self.themes.len() - 1 {
                themes.push_str("\n");
            }
        }
        themes.push_str("}");
        themes
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    background: ThemeColor,
    foreground: ThemeColor,
    primary: ThemeColor,
    primary_foreground: ThemeColor,
    secondary: ThemeColor,
    secondary_foreground: ThemeColor,
    accent: ThemeColor,
    accent_foreground: ThemeColor,
    muted: ThemeColor,
    muted_foreground: ThemeColor,
    destructive: ThemeColor,
    destructive_foreground: ThemeColor,
    success: ThemeColor,
    success_foreground: ThemeColor,
    border: ThemeColor,
    input: ThemeColor,
    ring: ThemeColor,
    radius: RadiusCss,
}

impl std::default::Default for Theme {
    fn default() -> Self {
        Self {
            name: "light".to_string(),
            background: ThemeColor::Background(HslColor {
                h: 0,
                s: 0.0,
                l: 100.0,
            }),
            foreground: ThemeColor::Foreground(HslColor {
                h: 240,
                s: 0.1,
                l: 3.9,
            }),
            primary: ThemeColor::Primary(HslColor {
                h: 292,
                s: 1.0,
                l: 30.0,
            }),
            primary_foreground: ThemeColor::PrimaryForeground(HslColor {
                h: 355,
                s: 1.0,
                l: 97.3,
            }),
            secondary: ThemeColor::Secondary(HslColor {
                h: 123,
                s: 1.0,
                l: 22.0,
            }),
            secondary_foreground: ThemeColor::SecondaryForeground(HslColor {
                h: 123,
                s: 1.0,
                l: 70.0,
            }),
            accent: ThemeColor::Accent(HslColor {
                h: 240,
                s: 0.048,
                l: 95.9,
            }),
            accent_foreground: ThemeColor::AccentForeground(HslColor {
                h: 240,
                s: 0.059,
                l: 10.0,
            }),
            muted: ThemeColor::Muted(HslColor {
                h: 240,
                s: 0.048,
                l: 95.9,
            }),
            muted_foreground: ThemeColor::MutedForeground(HslColor {
                h: 240,
                s: 0.038,
                l: 46.1,
            }),
            destructive: ThemeColor::Destructive(HslColor {
                h: 0,
                s: 0.842,
                l: 60.2,
            }),
            destructive_foreground: ThemeColor::DestructiveForeground(HslColor {
                h: 0,
                s: 0.0,
                l: 98.0,
            }),
            success: ThemeColor::Success(HslColor {
                h: 142,
                s: 0.7,
                l: 45.0,
            }),
            success_foreground: ThemeColor::SuccessForeground(HslColor {
                h: 120,
                s: 1.0,
                l: 97.3,
            }),
            border: ThemeColor::Border(HslColor {
                h: 240,
                s: 0.059,
                l: 90.0,
            }),
            input: ThemeColor::Input(HslColor {
                h: 240,
                s: 0.059,
                l: 90.0,
            }),
            ring: ThemeColor::Ring(HslColor {
                h: 240,
                s: 0.1,
                l: 3.9,
            }),
            radius: RadiusCss("5px".to_string()),
        }
    }
}

impl Theme {
    pub fn dark_default() -> Self {
        Self {
            name: "dark".to_string(),
            background: ThemeColor::Background(HslColor {
                h: 0,
                s: 0.0,
                l: 0.0,
            }),
            foreground: ThemeColor::Foreground(HslColor {
                h: 240,
                s: 0.1,
                l: 97.3,
            }),
            primary: ThemeColor::Primary(HslColor {
                h: 292,
                s: 1.0,
                l: 30.0,
            }),
            primary_foreground: ThemeColor::PrimaryForeground(HslColor {
                h: 355,
                s: 1.0,
                l: 97.3,
            }),
            secondary: ThemeColor::Secondary(HslColor {
                h: 123,
                s: 1.0,
                l: 22.0,
            }),
            secondary_foreground: ThemeColor::SecondaryForeground(HslColor {
                h: 123,
                s: 1.0,
                l: 70.0,
            }),
            accent: ThemeColor::Accent(HslColor {
                h: 240,
                s: 0.048,
                l: 95.9,
            }),
            accent_foreground: ThemeColor::AccentForeground(HslColor {
                h: 240,
                s: 0.059,
                l: 10.0,
            }),
            muted: ThemeColor::Muted(HslColor {
                h: 240,
                s: 0.048,
                l: 95.9,
            }),
            muted_foreground: ThemeColor::MutedForeground(HslColor {
                h: 240,
                s: 0.038,
                l: 46.1,
            }),
            destructive: ThemeColor::Destructive(HslColor {
                h: 0,
                s: 0.842,
                l: 60.2,
            }),
            destructive_foreground: ThemeColor::DestructiveForeground(HslColor {
                h: 0,
                s: 0.0,
                l: 98.0,
            }),
            success: ThemeColor::Success(HslColor {
                h: 142,
                s: 0.7,
                l: 45.0,
            }),
            success_foreground: ThemeColor::SuccessForeground(HslColor {
                h: 120,
                s: 1.0,
                l: 97.3,
            }),
            border: ThemeColor::Border(HslColor {
                h: 240,
                s: 0.059,
                l: 90.0,
            }),
            input: ThemeColor::Input(HslColor {
                h: 240,
                s: 0.059,
                l: 90.0,
            }),
            ring: ThemeColor::Ring(HslColor {
                h: 240,
                s: 0.1,
                l: 97.3,
            }),
            radius: RadiusCss("5px".to_string()),
        }
    }

    pub fn to_style(&self) -> String {
        format!(
            r#"  --background: {};
  --foreground: {};
  --primary: {};
  --primary-foreground: {};
  --secondary: {};
  --secondary-foreground: {};
  --accent: {};
  --accent-foreground: {};
  --muted: {};
  --muted-foreground: {};
  --destructive: {};
  --destructive-foreground: {};
  --success: {};
  --success-foreground: {};
  --border: {};
  --input: {};
  --ring: {};
  --global-radius: {};"#,
            self.background.to_style(),
            self.foreground.to_style(),
            self.primary.to_style(),
            self.primary_foreground.to_style(),
            self.secondary.to_style(),
            self.secondary_foreground.to_style(),
            self.accent.to_style(),
            self.accent_foreground.to_style(),
            self.muted.to_style(),
            self.muted_foreground.to_style(),
            self.destructive.to_style(),
            self.destructive_foreground.to_style(),
            self.success.to_style(),
            self.success_foreground.to_style(),
            self.border.to_style(),
            self.input.to_style(),
            self.ring.to_style(),
            self.radius.0,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThemeColor {
    Background(HslColor),
    Foreground(HslColor),
    Primary(HslColor),
    PrimaryForeground(HslColor),
    Secondary(HslColor),
    SecondaryForeground(HslColor),
    Accent(HslColor),
    AccentForeground(HslColor),
    Muted(HslColor),
    MutedForeground(HslColor),
    Destructive(HslColor),
    DestructiveForeground(HslColor),
    Success(HslColor),
    SuccessForeground(HslColor),
    Border(HslColor),
    Input(HslColor),
    Ring(HslColor),
}

impl ThemeColor {
    pub fn to_style(&self) -> String {
        match self {
            ThemeColor::Background(color)
            | ThemeColor::Foreground(color)
            | ThemeColor::Primary(color)
            | ThemeColor::PrimaryForeground(color)
            | ThemeColor::Secondary(color)
            | ThemeColor::SecondaryForeground(color)
            | ThemeColor::Accent(color)
            | ThemeColor::AccentForeground(color)
            | ThemeColor::Muted(color)
            | ThemeColor::MutedForeground(color)
            | ThemeColor::Destructive(color)
            | ThemeColor::DestructiveForeground(color)
            | ThemeColor::Success(color)
            | ThemeColor::SuccessForeground(color)
            | ThemeColor::Border(color)
            | ThemeColor::Input(color)
            | ThemeColor::Ring(color) => color.to_style(),
        }
    }
}

impl std::fmt::Display for ThemeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeColor::Background(_) => write!(f, "background"),
            ThemeColor::Foreground(_) => write!(f, "foreground"),
            ThemeColor::Primary(_) => write!(f, "primary"),
            ThemeColor::PrimaryForeground(_) => write!(f, "primary-foreground"),
            ThemeColor::Secondary(_) => write!(f, "secondary"),
            ThemeColor::SecondaryForeground(_) => write!(f, "secondary-foreground"),
            ThemeColor::Accent(_) => write!(f, "accent"),
            ThemeColor::AccentForeground(_) => write!(f, "accent-foreground"),
            ThemeColor::Muted(_) => write!(f, "muted"),
            ThemeColor::MutedForeground(_) => write!(f, "muted-foreground"),
            ThemeColor::Destructive(_) => write!(f, "destructive"),
            ThemeColor::DestructiveForeground(_) => write!(f, "destructive-foreground"),
            ThemeColor::Success(_) => write!(f, "success"),
            ThemeColor::SuccessForeground(_) => write!(f, "success-foreground"),
            ThemeColor::Border(_) => write!(f, "border"),
            ThemeColor::Input(_) => write!(f, "input"),
            ThemeColor::Ring(_) => write!(f, "ring"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HslColor {
    h: i64,
    s: f64,
    l: f64,
}

impl HslColor {
    pub fn to_style(&self) -> String {
        format!("{}deg {}% {}%", self.h, self.s, self.l)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadiusCss(String);

impl RadiusCss {
    pub fn to_style(&self) -> String {
        self.0.clone()
    }
}
