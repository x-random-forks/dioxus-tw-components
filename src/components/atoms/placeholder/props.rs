use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;

/// A simple placeholder component that can be used to show for example a loading state
#[props_component(id, class)]
pub fn Placeholder(
    /// Used to control the level of animation of the component
    #[props(default)]
    color: Color,
    #[props(default)]
    animation: Animation,
) -> Element {
    rsx!(
        div { class: props.class }
    )
}

impl Named for PlaceholderProps {
    const NAME: &'static str = "Placeholder";
}