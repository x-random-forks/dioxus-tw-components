use super::props::*;
use crate::attributes::*;

impl Class for RadioProps {
    fn base(&self) -> &'static str {
        "peer"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match self.color {
            Color::Primary => "accent-primary focus:ring-primary focus:ring-2 focus:ring-offset-1",
            Color::Secondary => {
                "accent-secondary focus:ring-secondary focus:ring-2 focus:ring-offset-1"
            }
            Color::Destructive => {
                "accent-destructive focus:ring-destructive focus:ring-2 focus:ring-offset-1"
            }
            Color::Success => "accent-success focus:ring-success focus:ring-2 focus:ring-offset-1",
            _ => "accent-foreground focus:ring-foreground focus:ring-2 focus:ring-offset-1",
        })
    }
}
