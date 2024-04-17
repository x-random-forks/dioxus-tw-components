use super::props::*;
use crate::types::*;

impl BaseClass for AccordionItemProps {
    fn base(&self) -> &'static str {
        "border-b"
    }
}

impl BaseClass for AccordionTriggerProps {
    fn base(&self) -> &'static str {
        "flex flex-1 items-center justify-between p-small w-full text-medium font-medium group transition-all hover:underline"
    }
}

impl BaseClass for AccordionContentProps {
    fn base(&self) -> &'static str {
        "text-small overflow-hidden transition-all px-small"
    }
}
