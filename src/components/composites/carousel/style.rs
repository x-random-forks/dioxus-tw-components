use super::props::*;
use crate::attributes::*;

impl BaseClass for CarouselProps {
    fn base(&self) -> &'static str {
        "container flex items-center"
    }
}

impl BaseClass for CarouselWindowProps {
    fn base(&self) -> &'static str {
        "relative overflow-hidden grow rounded-global-radius border border-border"
    }
}

impl BaseClass for CarouselContentProps {
    fn base(&self) -> &'static str {
        "flex aspect-square"
    }
}

impl Animatable for CarouselContentProps {
    fn animation(&self) -> &'static str {
        match self.animation {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transform transition-transform duration-500",
            Animation::Custom(animation) => animation,
        }
    }
}

impl BaseClass for CarouselItemProps {
    fn base(&self) -> &'static str {
        "relative min-w-0 shrink-0 grow-0 basis-full p-medium"
    }
}

impl BaseClass for CarouselTriggerProps {
    fn base(&self) -> &'static str {
        "size-10 flex flex-nowrap items-center justify-center"
    }
}
