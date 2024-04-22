use super::props::*;
use crate::types::*;

impl BaseClass for DropdownProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl BaseClass for DropdownToggleProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl BaseClass for DropdownContentProps {
    // REVIEW split base class into multiple function, like position, size, padding, margin, text, ... ?
    fn base(&self) -> &'static str {
        "absolute z-10 shadow-sm bg-background rounded-global-radius border border-border my-1 p-2 whitespace-nowrap transition-all duration-100 shadow opacity-100 data-[state=inactive]:opacity-0 data-[state=inactive]:invisible data-[state=inactive]:scale-90 data-[state=active]:scale-100"
    }
}
