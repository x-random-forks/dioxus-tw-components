use super::props::*;
use crate::types::*;

impl BaseClass for DropdownProps {
    fn base(&self) -> &'static str {
        "relative group"
    }
}

impl BaseClass for DropdownToggleProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl BaseClass for DropdownContentProps {
    fn base(&self) -> &'static str {
        "absolute hidden z-10 group-focus-within:block shadow-sm bg-background rounded-global-radius border border-border my-1 p-2 whitespace-nowrap"
    }
}
