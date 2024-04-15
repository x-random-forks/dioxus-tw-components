use dioxus::prelude::*;
use myderive::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(class, id)]
pub fn Navbar(
    #[props(default)] left_part: Element,
    #[props(default)] right_part: Element,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        nav { class: class, id: props.id,
            div { class: "flex space-x-2 items-center ml-6", {props.left_part} }
            div { class: "flex flex-1 items-center justify-end space-x-2 mr-6", {props.right_part} }
        }
    )
}
