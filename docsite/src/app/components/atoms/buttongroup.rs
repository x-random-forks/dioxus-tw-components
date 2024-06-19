use dioxus::prelude::*;
use dioxus_components::atoms::buttongroup::{ButtonGroup, ButtonGroupItem, ButtonGroupProps};

use crate::app::doctrait::DemoComponent;

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

    fn BuildCompPreview() -> Element {
        rsx!(
            ButtonGroup { 
                ButtonGroupItem { "A" }
                ButtonGroupItem { "BADF" }
            }
        )        
    }

    fn BuildCompSelectors() -> Element {
        rsx!(  )
    }
}