use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(class, children, id)]
pub fn Scrollable(#[props(default = Orientation::Vertical)] orientation: Orientation) -> Element {
    rsx!(
        div { class: props.class, id: props.id, {props.children} }
    )
}
