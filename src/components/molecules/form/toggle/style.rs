use super::props::*;
use crate::attributes::*;

impl BaseClass for ToggleProps {
    fn base(&self) -> &'static str {
        "peer relative bg-input rounded-full focus:outline-none focus:ring-2 focus:ring-black focus:ring-offset-2 data-[state=inactive]:after:translate-x-full data-[state=inactive]:after:border-white after:content-[''] after:absolute after:bg-background after:border-input after:border after:rounded-full after:transition-all transition-colors duration-100"
    }
}

impl Colorable for ToggleProps {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Default => "data-[state=inactive]:bg-foreground",
            Color::Primary => "data-[state=inactive]:bg-primary",
            Color::Secondary => "data-[state=inactive]:bg-secondary",
            Color::Destructive => "data-[state=inactive]:bg-destructive",
            Color::Success => "data-[state=inactive]:bg-success",
            _ => "",
        }
    }
}

impl Sizable for ToggleProps {
    fn size(&self) -> &'static str {
        match self.size {
            Size::Lg => "w-14 h-7 after:top-0.5 after:start-[4px] after:h-6 after:w-6",
            Size::Md => "w-11 h-6 after:top-[2px] after:start-[2px] after:h-5 after:w-5",
            Size::Sm => "w-9 h-5 after:top-[2px] after:start-[2px] after:h-4 after:w-4",
            _ => "",
        }
    }
}
