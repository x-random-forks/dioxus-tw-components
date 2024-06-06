use super::props::*;
use crate::attributes::*;

impl Class for AccordionProps {}

impl Class for AccordionItemProps {
    fn base(&self) -> &'static str {
        "border-b"
    }
}

impl Class for AccordionTriggerProps {
    fn base(&self) -> &'static str {
        "flex flex-1 items-center justify-between p-extrasmall w-full d-text-small font-medium group hover:underline"
    }
}

impl Class for AccordionContentProps {
    fn base(&self) -> &'static str {
        "d-text-small overflow-hidden px-small"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transition-all",
            Animation::Custom(animation) => animation,
        })
    }
}
