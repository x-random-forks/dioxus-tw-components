use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(class, id)]
pub fn Navbar(#[props(default)] children: Element) -> Element {
    rsx!(
        nav { class: props.class, id: props.id, {props.children} }
    )
}
