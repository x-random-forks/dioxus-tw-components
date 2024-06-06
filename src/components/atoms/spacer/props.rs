use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
// Will have to use to see if it's practical or not, not convinced yet
#[props_component(class)]
pub fn Spacer() -> Element {
    rsx!(
        div { class: props.class }
    )
}
