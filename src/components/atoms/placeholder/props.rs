use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;

/// A simple placeholder component that can be used to show for example a loading state
#[props_component(id, class)]
pub fn Placeholder(
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Used to control the level of animation of the component
    #[props(default)]
    color: Color,
    #[props(default)]
    animation: Animation,
) -> Element {
    rsx!(
        div { ..props.attributes, class: props.class }
    )
}
