use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(id, class)]
pub fn Placeholder(#[props(default)] animation: super::PlaceholderAnimation) -> Element {
    let class = tw_merge!(props.base(), props.variant(), props.class);

    rsx!( div { class: class } )
}
