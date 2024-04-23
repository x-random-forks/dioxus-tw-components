use super::props::*;
use crate::attributes::*;

impl BaseClass for SeparatorProps {
    fn base(&self) -> &'static str {
        "bg-border shrink-0"
    }
}

impl Orientable for SeparatorProps {
    fn orientation(&self) -> &'static str {
        match self.orientation {
            Orientation::Horizontal => "w-full h-[1px]",
            Orientation::Vertical => "h-full w-[1px]",
        }
    }
}
