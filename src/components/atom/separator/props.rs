use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

/// A simple separator component
#[props_component(class)]
pub fn Separator(
    /// Handle the orientation of the separator
    #[props(default = Orientation::Horizontal)]
    orientation: Orientation,
) -> Element {
    let class = tw_merge!(props.base(), props.orientation(), props.class);

    rsx!( div { class: class } )
}
