use super::props::*;
use crate::attributes::*;

impl BaseClass for AccordionItemProps {
    fn base(&self) -> &'static str {
        "border-b"
    }
}

impl BaseClass for AccordionTriggerProps {
    fn base(&self) -> &'static str {
        "flex flex-1 items-center justify-between p-extrasmall w-full d-text-small font-medium group hover:underline"
    }
}

impl BaseClass for AccordionContentProps {
    fn base(&self) -> &'static str {
        "d-text-small overflow-hidden px-small"
    }
}

impl Animatable for AccordionContentProps {
    fn animation(&self) -> &'static str {
        match self.animation {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transition-all",
            Animation::Custom(animation) => animation,
        }
    }
}
