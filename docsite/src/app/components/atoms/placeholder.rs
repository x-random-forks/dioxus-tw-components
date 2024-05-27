use dioxus::prelude::*;
use dioxus_components::{atoms::{placeholder::PlaceholderProps, Placeholder}, attributes::Animation};

use crate::app::{components::preview::{PreviewClass, PreviewFull, PreviewGroupAttr}, doctrait::{DemoAttribute, DemoComp, DemoState}};

#[component]
pub fn PlaceholderPage() -> Element {
    rsx!(
        PreviewFull::<PlaceholderProps> {}
    )
}

impl DemoComp for PlaceholderProps {
    fn title() -> &'static str {
        "Placeholder"
    }

    fn preview_comp(demo_state: &DemoState) -> Element {
        rsx!(
            Placeholder {
                animation: demo_state.get_animation()(),
                class: demo_state.get_custom_class()(),
                override_class: demo_state.get_override_class()()
            }
        )
    }

    fn select_attributes(demo_state: &DemoState) -> Element {
        rsx!(
            PreviewClass {
                signal_class: demo_state.get_custom_class(),
                signal_override: demo_state.get_override_class()
            }
            PreviewGroupAttr { {Animation::demo_attr(demo_state.get_animation())} }
        )
    }
}

