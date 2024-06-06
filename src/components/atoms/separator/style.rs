use super::props::*;
use crate::attributes::*;

impl Class for SeparatorProps {
    fn base(&self) -> &'static str {
        "bg-border shrink-0"
    }

    fn orientation(&self) -> Option<&'static str> {
        Some(match self.orientation {
            Orientation::Horizontal => "w-full h-[1px]",
            Orientation::Vertical => "h-full w-[1px]",
        })
    }
}
