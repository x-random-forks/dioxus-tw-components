use super::props::*;
use crate::types::*;

impl BaseClass for PlaceholderProps {
    fn base(&self) -> &'static str {
        "bg-foreground/20 rounded-global-radius"
    }
}

impl Animatable for PlaceholderProps {
    fn animation(&self) -> &'static str {
        match self.animation {
            Animation::None => "",
            Animation::Light => "animate-pulse",
            Animation::Full => "relative overflow-hidden before:absolute before:inset-0 before:-translate-x-full before:bg-gradient-to-r before:from-transparent before:via-white/40 before:animate-[shimmer_2s_infinite]",
            Animation::Custom(animation) => animation,
        }
    }
}
