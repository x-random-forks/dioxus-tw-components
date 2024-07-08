use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for FormListProps {
    fn base(&self) -> &'static str {
        "text-foreground"
    }
}

impl Class for FormListTriggerPlusProps {}

impl Class for FormListTriggerMinusProps {}

impl Class for FormListContentProps {
    fn base(&self) -> &'static str {
        "transition-all overflow-y-hidden"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => "transition-all",
        })
    }
}
