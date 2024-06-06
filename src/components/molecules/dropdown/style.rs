use super::props::*;
use crate::attributes::*;

impl Class for DropdownProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl Class for DropdownToggleProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl Class for DropdownContentProps {
    fn base(&self) -> &'static str {
        "absolute z-10 shadow shadow-sm bg-background rounded-global-radius border border-border my-1 p-small whitespace-nowrap opacity-100 data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transition-all duration-100 data-[state=inactive]:scale-90 data-[state=active]:scale-100 data-[state=inactive]:opacity-0",
            Animation::Custom(animation) => animation,
        })
    }
}
