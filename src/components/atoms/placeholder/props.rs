use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

/// A simple placeholder component that can be used to show for example a loading state
#[props_component(id, class)]
pub fn Placeholder(
    /// Used to control the level of animation of the component
    #[props(default)]
    animation: Animation,
) -> Element {
    let class_str = tw_merge!(
        props.base(),
        props.animation(),
        props.class
    );

    rsx!(
        div { class: class_str }
    )
}
