use super::props::*;
use crate::attributes::*;

impl BaseClass for RadioProps {
    fn base(&self) -> &'static str {
        "peer"
    }
}

impl Colorable for RadioProps {
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
            Color::Default | _ => {
                "accent-foreground focus:ring-foreground focus:ring-2 focus:ring-offset-1"
            }
        }
    }
}
