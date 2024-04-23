use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

// Will have to use to see if it's practical or not, not convinced yet
#[props_component(class)]
pub fn Spacer() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class }
    )
}
