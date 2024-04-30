use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(class, children)]
pub fn FormDesc() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        span { class, {props.children} }
    )
}
