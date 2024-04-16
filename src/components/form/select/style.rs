use super::props::*;
use crate::types::*;

impl BaseClass for SelectGroupProps {
    fn base(&self) -> &'static str {
        "flex flex-col w-full bg-input"
    }
}

impl BaseClass for SelectPlaceholderProps {
    fn base(&self) -> &'static str {
        "hidden"
    }
}

impl BaseClass for SelectLabelProps {
    fn base(&self) -> &'static str {
        "px-2 py-1.5 text-sm font-semibold"
    }
}

impl BaseClass for SelectItemProps {
    fn base(&self) -> &'static str {
        "flex py-1.5 pl-2 pr-8 text-sm"
    }
}
