use super::props::*;
use crate::attributes::*;

impl BaseClass for TextAreaProps {
    fn base(&self) -> &'static str {
        "flex w-full px-3 py-2 text-sm text-foreground bg-background border rounded-global-radius hover:brightness-105 focus:brightness-105 disabled:bg-muted disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100"
    }
}

impl Colorable for TextAreaProps {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Primary => "border-primary",
            Color::Secondary => "border-secondary",
            Color::Destructive => "border-destructive",
            Color::Success => "border-success",
            Color::Default | _ => "border-input",
        }
    }
}
