use super::CheckboxProps;
use crate::styling::{BaseClass, Color};

impl std::fmt::Display for BaseClass<CheckboxProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => "peer",
            _ => "",
        };
        write!(f, "{}", class)
    }
}

impl std::fmt::Display for Color<CheckboxProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Color::Primary => "accent-primary focus:ring-primary focus:ring-2 focus:ring-offset-1",
            Color::Secondary => {
                "accent-secondary focus:ring-secondary focus:ring-2 focus:ring-offset-1"
            }
            Color::Accent => {
                "accent-accent focus:ring-accent-foreground focus:ring-2 focus:ring-offset-1"
            }
            Color::DefaultColor | _ => "",
        };
        write!(f, "{}", size)
    }
}
