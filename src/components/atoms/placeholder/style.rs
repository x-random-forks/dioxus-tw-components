use super::props::*;
use crate::attributes::*;

impl Class for PlaceholderProps {
    fn base(&self) -> &'static str {
        "bg-foreground/50 rounded-global-radius w-24 h-24"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::None => "",
            Animation::Light => "animate-pulse",
            Animation::Full => "relative overflow-hidden before:absolute before:inset-0 before:-translate-x-full before:bg-gradient-to-r before:from-transparent before:via-white/40 before:animate-[shimmer_2s_infinite]",
            Animation::Custom(animation) => animation,
        })
    }
}
