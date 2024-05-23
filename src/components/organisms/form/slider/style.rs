use super::props::*;
use crate::attributes::*;

impl BaseClass for SliderProps {
    fn base(&self) -> &'static str {
        "w-full disabled:cursor-not-allowed disabled:opacity-50"
    }
}

impl Colorable for SliderProps {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Default => "accent-foreground",
            Color::Primary => "accent-primary",
            Color::Secondary => "accent-secondary",
            Color::Destructive => "accent-destructive",
            Color::Success => "accent-success",
            Color::Accent => "accent-accent",
            Color::Muted => "accent-muted",
        }
    }
}

impl BaseClass for SliderLabelProps {
    fn base(&self) -> &'static str {
        "text-xs"
    }
}