use super::ButtonProps;
use crate::styling::{BaseClass, Color, Size, Variant};

impl std::fmt::Display for BaseClass<ButtonProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => {
                "text-center \
            text-base font-medium \
            border border-1 \
            rounded-global-radius \
            shadow-global-shadow \
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

impl std::fmt::Display for Variant<ButtonProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Variant::Outline(color) => match color {
                Color::Primary => "border border-primary text-primary hover:bg-primary/10",
                Color::Secondary => "border border-secondary text-secondary hover:bg-secondary/10",
                Color::Accent => "border border-accent text-accent hover:bg-accent/10",
                _ => "",
            },
            Variant::Ghost(color) => match color {
                Color::Primary => "border-none text-primary hover:bg-primary/10",
                Color::Secondary => "border-none text-secondary hover:bg-secondary/10",
                Color::Accent => "border-none text-accent hover:bg-accent/10",
                _ => "",
            },
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
