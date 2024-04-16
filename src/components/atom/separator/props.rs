use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(class)]
pub fn Separator(#[props(default = Orientation::Horizontal)] orientation: Orientation) -> Element {
    let class = tw_merge!(props.base(), props.orientation(), props.class);

    rsx!( div { class: class } )
}
