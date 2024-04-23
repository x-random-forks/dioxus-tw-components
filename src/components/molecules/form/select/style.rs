use super::props::*;
use crate::attributes::*;

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
        "px-extrasmall py-extrasmall text-sm font-semibold"
    }
}

impl BaseClass for SelectItemProps {
    fn base(&self) -> &'static str {
        "flex py-extrasmall pl-2 pr-8 text-sm"
    }
}
