use super::props::*;
use crate::types::*;

impl BaseClass for ToggleProps {
    fn base(&self) -> &'static str {
        "relative bg-input rounded-full peer-disabled:bg-muted peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-black peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:bg-white after:border-input after:border after:rounded-full after:transition-all"
    }
}

impl Colorable for ToggleProps {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Primary => "peer-checked:bg-primary",
            Color::Secondary => "peer-checked:bg-secondary",
            Color::Destructive => "peer-checked:bg-destructive",
            Color::Success => "peer-checked:bg-success",
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
