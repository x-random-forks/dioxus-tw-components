use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;

/// A simple separator component
#[props_component(class)]
pub fn Separator(
    /// Handle the orientation of the separator
    #[props(default = Orientation::Horizontal)]
    orientation: Orientation,
) -> Element {
    rsx!(
        div { class: props.class }
    )
}
