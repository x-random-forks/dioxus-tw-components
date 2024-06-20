use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for PlaceholderProps {
    fn base(&self) -> &'static str {
        "rounded-global-radius w-24 h-24"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Default => "bg-foreground/50",
            Color::Primary => "bg-primary/50",
            Color::Secondary => "bg-secondary/50",
            Color::Destructive => "bg-destructive/50",
            Color::Success => "bg-success/50",
            Color::Accent => "bg-accent/50",
            Color::Muted => "bg-muted/50",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light => "animate-pulse",
            Animation::Full => "relative overflow-hidden before:absolute before:inset-0 before:-translate-x-full before:bg-gradient-to-r before:from-transparent before:via-white/40 before:animate-[shimmer_2s_infinite]",
            Animation::Custom(animation) => animation,
        })
    }
}
