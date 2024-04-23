use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(class, id)]
pub fn Navbar(#[props(default)] children: Element) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        nav { class: class, id: props.id, {props.children} }
    )
}
