use super::LabelProps;
use crate::styling::{BaseClass, Color};

impl std::fmt::Display for BaseClass<LabelProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => "font-medium",
            _ => "",
        };
        write!(f, "{}", class)
    }
}

impl std::fmt::Display for Color<LabelProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Color::Primary => "text-primary peer-disabled:text-muted",
            Color::Secondary => "text-secondary peer-disabled:text-muted",
            Color::Accent => "text-accent peer-disabled:text-muted",
            _ => "text-foreground peer-disabled:text-muted",
        };
        write!(f, "{}", size)
    }
}
