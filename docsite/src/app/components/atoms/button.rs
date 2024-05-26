use dioxus::prelude::*;
use dioxus_components::{
    atoms::{button::ButtonProps, Button, ButtonVariant},
    attributes::{Animation, Color, Size},
};

use crate::app::{
    components::preview::{PreviewCustomClass, PreviewFull, PreviewGroupAttr},
    doctrait::{DemoAttribute, DemoComp, DemoState},
};

#[component]
pub fn ButtonPage() -> Element {
    rsx!(
        PreviewFull::<ButtonProps> {}
    )
}

impl DemoComp for ButtonProps {
    fn title() -> &'static str {
        "Button"
    }

    fn preview_comp(demo_state: &DemoState) -> Element {
        rsx!(
            Button {
                color: demo_state.get_color()(),
                size: demo_state.get_size()(),
                variant: demo_state.get_variant()(),
                animation: demo_state.get_animation()(),
                class: demo_state.get_custom_class()(),
                "Button"
            }
        )
    }

    fn select_attributes(demo_state: &DemoState) -> Element {
        rsx!(
            PreviewCustomClass { signal: demo_state.get_custom_class() }
            PreviewGroupAttr { 
                {Color::demo_attr(demo_state.get_color())},
                {Size::demo_attr(demo_state.get_size())},
                {ButtonVariant::demo_attr(demo_state.get_variant())},
                {Animation::demo_attr(demo_state.get_animation())}
            }
        )
    }
}
