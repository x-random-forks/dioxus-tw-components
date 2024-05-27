use super::props::*;
use crate::attributes::*;

impl BaseClass for SelectGroupProps {
    fn base(&self) -> &'static str {
        "flex flex-col w-full bg-background border border-border px-3 py-2 rounded-global-radius text-foreground"
    }
}

impl BaseClass for SelectPlaceholderProps {
    fn base(&self) -> &'static str {
        "hidden font-bold"
    }
}

impl BaseClass for SelectLabelProps {
    fn base(&self) -> &'static str {
        "text-sm font-semibold"
    }
}

impl BaseClass for SelectItemProps {
    fn base(&self) -> &'static str {
        "flex pl-2 pr-8 text-sm"
    }
}
