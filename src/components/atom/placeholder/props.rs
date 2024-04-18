use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

/// A simple placeholder component that can be used to show for example a loading state
#[props_component(id, class)]
pub fn Placeholder(
    /// Used to control the level of animation of the component
    #[props(default)]
    animation: Animation,
    /// This use tailwind w-{width} h-{height} classes so not every value is possible
    #[props(default = 24)]
    width: u32,
    /// This use tailwind w-{width} h-{height} classes so not every value is possible
    #[props(default = 4)]
    height: u32,
) -> Element {
    let class = tw_merge!(
        props.base(),
        props.animation(),
        format!("w-{}", props.width),
        format!("h-{}", props.height),
        props.class
    );

    rsx!( div { class: class } )
}
