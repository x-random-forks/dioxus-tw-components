use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(class, children, id)]
pub fn HeaderTemplate() -> Element {
    rsx!(
        header { class: props.class, id: props.id, { props.children } }
    )
}

impl Class for HeaderTemplateProps {}