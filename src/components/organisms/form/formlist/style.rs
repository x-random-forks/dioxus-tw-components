use super::props::*;
use crate::attributes::*;

impl Class for FormListProps {}

impl Class for FormListTitleProps {
    fn base(&self) -> &'static str {
        "h4"
    }
}

impl Class for FormListTriggerProps {
    fn base(&self) -> &'static str {
        "size-10 border inline-flex place-content-center place-items-center"
    }
}

impl Class for FormListContentProps {}
