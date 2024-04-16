use super::props::*;
use crate::types::*;

impl BaseClass for AccordionItemProps {
    fn base(&self) -> &'static str {
        "border-b"
    }
}

impl BaseClass for AccordionTriggerProps {
    fn base(&self) -> &'static str {
        "flex flex-1 items-center justify-between py-2 font-medium transition-all hover:underline"
    }
}

impl BaseClass for AccordionContentProps {
    fn base(&self) -> &'static str {
        "text-sm overflow-hidden transition-all"
    }
}
