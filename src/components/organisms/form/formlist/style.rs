use super::props::*;
use crate::attributes::*;

impl BaseClass for FormListProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl BaseClass for FormListTitleProps {
    fn base(&self) -> &'static str {
        "h4 grow"
    }
}

impl BaseClass for FormListTriggerProps {
    fn base(&self) -> &'static str {
        "size-10 border inline-flex place-content-center place-items-center ml-auto"
    }
}

impl BaseClass for FormListContentProps {
    fn base(&self) -> &'static str {
        "pl-2 pb-4"
    }
}