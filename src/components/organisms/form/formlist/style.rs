use super::props::*;
use crate::attributes::*;

impl BaseClass for FormListProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl BaseClass for FormListTitleProps {
    fn base(&self) -> &'static str {
        "h4"
    }
}

impl BaseClass for FormListTriggerProps {
    fn base(&self) -> &'static str {
        "size-10 border inline-flex place-content-center place-items-center"
    }
}

impl BaseClass for FormListContentProps {
    fn base(&self) -> &'static str {
        ""
    }
}