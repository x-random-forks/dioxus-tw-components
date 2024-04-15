use dioxus::prelude::*;
use myderive::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(id, class)]
pub fn Placeholder(
    #[props(default)] radius: super::PlaceholderRadius,
    #[props(default = true)] animation: bool,
) -> Element {
    let class = tw_merge!(
        props.base(),
        props.variant(),
        if props.animation { "animate-pulse" } else { "" },
        props.class
    );

    rsx!( div { class: class } )
}
