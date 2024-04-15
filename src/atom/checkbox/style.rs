use super::props::*;
use crate::types::*;

impl BaseClass for CheckboxProps {
    fn base(&self) -> &'static str {
        "peer"
    }
}

impl Colorable for CheckboxProps {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Primary => "accent-primary focus:ring-primary focus:ring-2 focus:ring-offset-1",
            Color::Secondary => {
                "accent-secondary focus:ring-secondary focus:ring-2 focus:ring-offset-1"
            }
            Color::Destructive => {
                "accent-destructive focus:ring-destructive focus:ring-2 focus:ring-offset-1"
            }
            Color::Success => "accent-success focus:ring-success focus:ring-2 focus:ring-offset-1",
            _ => "accent-primary focus:ring-primary focus:ring-2 focus:ring-offset-1",
        }
    }
}
