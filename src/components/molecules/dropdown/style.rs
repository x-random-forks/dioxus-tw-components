use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for DropdownProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl Class for DropdownToggleProps {
    fn base(&self) -> &'static str {
        "px-4 py-2 text-sm font-medium text-foreground bg-background border border-input rounded-global-radius whitespace-nowrap cursor-pointer hover:bg-accent hover:text-accent-foreground"
    }
}

impl Class for DropdownContentProps {
    fn base(&self) -> &'static str {
        "absolute z-50 p-2 mt-1 space-y-2 min-w-[8rem] bg-background text-foreground rounded-global-radius border border-input shadow shadow-global-shadow whitespace-nowrap opacity-100 data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transition-all duration-100 data-[state=inactive]:scale-90 data-[state=active]:scale-100 data-[state=inactive]:opacity-0",
        })
    }
}
