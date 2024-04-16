use super::props::*;
use crate::types::*;

impl BaseClass for RadioGroupProps {
    fn base(&self) -> &'static str {
        "flex flex-col"
    }
}

// TODO
impl BaseClass for RadioItemProps {
    fn base(&self) -> &'static str {
        ""
    }
}
