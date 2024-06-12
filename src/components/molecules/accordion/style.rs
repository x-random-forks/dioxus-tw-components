use super::props::*;
use crate::attributes::*;

impl Class for AccordionProps {}

impl Class for AccordionItemProps {
    fn base(&self) -> &'static str {
        "border-b border-border"
    }
}

impl Class for AccordionTriggerProps {
    fn base(&self) -> &'static str {
        "flex flex-1 items-center justify-between py-2 space-x-2 font-medium text-foreground group hover:underline"
    }
}

impl Class for AccordionContentProps {
    fn base(&self) -> &'static str {
        "text-sm text-foreground overflow-hidden"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transition-all",
            Animation::Custom(animation) => animation,
        })
    }
}
