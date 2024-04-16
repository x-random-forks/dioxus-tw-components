use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(id, class)]
pub fn Placeholder(
    #[props(default)] animation: super::PlaceholderAnimation,
    #[props(default = 24)] width: u32,
    #[props(default = 4)] height: u32,
) -> Element {
    let class = tw_merge!(
        props.base(),
        props.variant(),
        format!("w-{}", props.width),
        format!("h-{}", props.height),
        props.class
    );

    rsx!( div { class: class } )
}
