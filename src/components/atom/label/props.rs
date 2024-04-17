use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(class, children)]
pub fn Label(#[props(default)] r#for: String) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        label { class: class, r#for: props.r#for, { props.children } }
    )
}
