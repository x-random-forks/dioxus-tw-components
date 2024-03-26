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
            Color::Primary => "text-primary",
            Color::Secondary => "text-secondary",
            Color::Accent => "text-accent",
            _ => "",
        };
        write!(f, "{}", size)
    }
}
