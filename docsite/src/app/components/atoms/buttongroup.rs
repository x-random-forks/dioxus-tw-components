use dioxus::prelude::*;
use dioxus_components::atoms::buttongroup::{ButtonGroup, ButtonGroupItem, ButtonGroupProps};

use crate::app::{components::preview::PreviewFull, doctrait::DemoComp};

pub fn ButtonGroupPage() -> Element {
    rsx!(
        PreviewFull::<ButtonGroupProps> {}
    )
}

impl DemoComp for ButtonGroupProps {
    fn title() -> &'static str {
        "Button Group"
    }

    fn preview_comp(demo_state: &crate::app::doctrait::DemoState) -> Element {
        rsx!(
            ButtonGroup { 
                ButtonGroupItem { "A" }
                ButtonGroupItem { "BADF" }
            }
        )        
    }

    fn select_attributes(demo_state: &crate::app::doctrait::DemoState) -> Element {
        rsx!(  )
    }
}