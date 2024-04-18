use super::props::*;
use crate::types::*;

impl BaseClass for PlaceholderProps {
    fn base(&self) -> &'static str {
        "bg-foreground/20 rounded-global-radius"
    }
}

/// Used to control the level of animation
#[derive(Default, Clone, Copy, PartialEq)]
pub enum PlaceholderAnimation {
    None,
    /// A light pulsing animation
    Light,
    /// A full shimmering animation
    #[default]
    Full,
}

impl Variation for PlaceholderProps {
    fn variant(&self) -> &'static str {
        match self.animation {
            PlaceholderAnimation::None => "rounded",
            PlaceholderAnimation::Light => "animate-pulse",
            PlaceholderAnimation::Full => "relative overflow-hidden before:absolute before:inset-0 before:-translate-x-full before:bg-gradient-to-r before:from-transparent before:via-white/30 before:animate-[shimmer_2s_infinite]",
        }
    }
}
