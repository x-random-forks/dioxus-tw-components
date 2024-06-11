use std::str::FromStr;

use super::props::*;
use crate::attributes::*;

impl Class for ButtonProps {
    fn base(&self) -> &'static str {
        "text-center font-medium rounded-global-radius shadow-global-shadow disabled:opacity-50 disabled:cursor-not-allowed"
    }

    // Handled in variant
    fn color(&self) -> Option<&'static str> {
        Some("")
    }

    fn size(&self) -> Option<&'static str> {
        Some(match self.size {
            Size::Xs => "px-2 py-1 text-sm",
            Size::Sm => "px-3 py-1.5 text-sm",
            Size::Md => "px-4 py-2 text-base",
            Size::Lg => "px-7 py-3 text-lg",
            Size::Xl => "px-9 py-4 text-xl",
        })
    }

    fn variant(&self) -> Option<&'static str> {
        Some(match self.variant {
            ButtonVariant::Default => match self.color {
                Color::Default => "bg-foreground text-background hover:bg-foreground/80 active:bg-foreground/70 active:shadow",
                Color::Primary => "bg-primary text-primary-foreground border-primary hover:bg-primary/90 active:bg-primary/80 active:shadow",
                Color::Secondary => "bg-secondary text-secondary-foreground border-secondary hover:bg-secondary/90 active:bg-secondary/80 active:shadow",
                Color::Destructive => "bg-destructive text-destructive-foreground border-destructive hover:bg-destructive/90 active:bg-destructive/80 active:shadow",
                Color::Success => "bg-success text-success-foreground border-success hover:bg-success/90 active:bg-success/80 active:shadow",
                Color::Accent => "bg-accent text-accent-foreground border-accent hover:bg-accent/90 active:bg-accent/80 active:shadow",
                Color::Muted => "bg-muted text-muted-foreground border-muted hover:bg-muted/90 active:bg-muted/80 active:shadow",
            },
            ButtonVariant::Outline => match self.color {
                Color::Default => {
                    "border bg-transparent border-foreground text-foreground hover:bg-foreground/40"
                }
                Color::Primary => "border bg-transparent border-primary text-primary hover:bg-primary/90 hover:text-primary-foreground",
                Color::Secondary => "border bg-transparent border-secondary text-secondary hover:bg-secondary/90 hover:text-secondary-foreground",
                Color::Destructive => "border bg-transparent border-destructive text-destructive hover:bg-destructive/90 hover:text-destructive-foreground",
                Color::Success => "border bg-transparent border-success text-success hover:bg-success/90 hover:text-success-foreground",
                Color::Accent => "border bg-transparent border-accent text-accent hover:bg-accent/90 hover:text-accent-foreground",
                Color::Muted => "border bg-transparent border-muted text-muted hover:bg-muted/90 hover:text-muted-foreground",
            },
            ButtonVariant::Ghost => match self.color {
                Color::Default => {
                    "bg-transparent border-foreground text-foreground hover:bg-foreground/40"
                }
                Color::Primary => "bg-transparent border-primary text-primary hover:bg-primary/90 hover:text-primary-foreground active:shadow",
                Color::Secondary => "bg-transparent border-secondary text-secondary hover:bg-secondary/90 hover:text-secondary-foreground active:shadow",
                Color::Destructive => "bg-transparent border-destructive text-destructive hover:bg-destructive/90 hover:text-destructive-foreground active:shadow",
                Color::Success => "bg-transparent border-success text-success hover:bg-success/90 hover:text-success-foreground active:shadow",
                Color::Accent => "bg-transparent border-accent text-accent hover:bg-accent/90 hover:text-accent-foreground active:shadow",
                Color::Muted => "bg-transparent border-muted text-muted hover:bg-muted/90 hover:text-muted-foreground active:shadow",
            },
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transition-colors duration-150",
            Animation::Custom(animation) => animation,
        })
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Ghost,
}

impl FromStr for ButtonVariant {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "outline" => Ok(ButtonVariant::Outline),
            "ghost" => Ok(ButtonVariant::Ghost),
            "default" | _ => Ok(ButtonVariant::Default),
        }
    }
}

impl std::fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ButtonVariant::Default => "Default",
            ButtonVariant::Outline => "Outline",
            ButtonVariant::Ghost => "Ghost  ",
        };
        f.write_str(s)
    }
}
