use super::ButtonProps;
use crate::styling::{BaseClass, Color, Size};

impl std::fmt::Display for BaseClass<ButtonProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => {
                "text-center \
            text-base font-medium \
            rounded-global-radius \
            shadow-global-shadow \
            transition-colors \
            duration-100 \
            disabled:opacity-50 disabled:cursor-not-allowed"
            }
            _ => "",
        };
        write!(f, "{}", class)
    }
}

impl std::fmt::Display for Color<ButtonProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Color::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
            Color::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/90",
            Color::Accent => "bg-accent text-accent-foreground hover:bg-accent/90",
            _ => "",
        };
        write!(f, "{}", size)
    }
}

impl std::fmt::Display for Size<ButtonProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::Md => "px-5 py-[9px] text-base",
            Size::Lg => "px-4 py-2 text-lg",
            Size::Sm => "px-3 py-2 text-sm",
            Size::Xl => "px-8 py-4 text-xl",
            Size::Xs => "px-2.5 py-1.5 text-xs",
            _ => panic!("Never construct a Phantom"),
        };
        write!(f, "{}", size)
    }
}

#[derive(Default, PartialEq, Clone)]
pub enum ButtonVariant {
    #[default]
    DefaultVariant,
    Ghost(Color),
    Outline(Color),
}

impl std::fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            ButtonVariant::Outline(color) => match color {
                Color::Primary => "border border-primary text-primary hover:bg-primary/80 hover:text-primary-foreground",
                Color::Secondary => "border border-secondary text-secondary hover:bg-secondary/80 hover:text-secondary-foreground",
                Color::Accent => "border border-accent text-accent hover:bg-accent/80 hover:text-accent-foreground",
                _ => "",
            },
            ButtonVariant::Ghost(color) => match color {
                Color::Primary => "border-none text-primary hover:bg-primary/80 hover:text-primary-foreground",
                Color::Secondary => "border-none text-secondary hover:bg-secondary/80 hover:text-secondary-foreground",
                Color::Accent => "border-none text-accent hover:bg-accent/80 hover:text-accent-foreground",
                _ => "",
            },
            _ => "",
        };
        write!(f, "{}", size)
    }
}
