use dioxus::prelude::*;
use dioxus_components::atoms::buttongroup::{ButtonGroup, ButtonGroupItem, ButtonGroupProps};

use crate::app::doctrait::DemoComponent;

#[component]
pub fn ButtonGroupPage() -> Element {
    rsx!(  )
}

impl DemoComponent for ButtonGroupProps {
    fn title() -> &'static str {
        "Button Group"
    }
    
    fn description() -> &'static str {
        ""
    }

    fn build_comp_preview() -> Element {
        rsx!(
            ButtonGroup { 
                ButtonGroupItem { "A" }
                ButtonGroupItem { "BADF" }
            }
        )        
    }

    fn build_comp_selectors() -> Element {
        rsx!(  )
    }
}