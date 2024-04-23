use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(class, children, id)]
pub fn Scrollable(#[props(default = Orientation::Vertical)] orientation: Orientation) -> Element {
    let class = tw_merge!(props.base(), props.orientation(), props.class);

    rsx!(
        div { class, id: props.id, {props.children} }
    )
}
