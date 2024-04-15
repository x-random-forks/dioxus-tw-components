use super::props::*;
use crate::types::*;

impl BaseClass for SeparatorProps {
    fn base(&self) -> &'static str {
        "bg-border shrink-0"
    }
}

impl Variation for SeparatorProps {
    fn variant(&self) -> &'static str {
        match self.vertical {
            true => "h-full w-[1px]",
            false => "w-full h-[1px]",
        }
    }
}
