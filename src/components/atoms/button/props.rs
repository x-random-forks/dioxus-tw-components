use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;

/// A simple button which you can use every HTML attributes on, and style based on variant, color and size
#[props_component(id, class, children)]
pub fn Button(
    /// Things like disabled, type,... see [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Callback when the button is clicked
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Style variant
    #[props(default)]
    variant: super::ButtonVariant,
    #[props(default)] color: Color,
    #[props(default)] size: Size,
    /// Control the on hover and on active effect animation
    #[props(default = Animation::Full)]
    animation: Animation,
) -> Element {
    let onclick = move |event| {
        if let Some(oc) = &props.onclick {
            oc.call(event)
        }
    };

    rsx!(
        button {
            ..props.attributes,
            class: props.class,
            id: props.id,
            onclick,
            {props.children}
        }
    )
}