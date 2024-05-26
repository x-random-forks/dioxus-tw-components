use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

/// A simple button which you can use every HTML attributes on, and style based on variant, color and size
#[props_component(id, class, children)]
pub fn Button(
    /// Things like disabled, type,... see [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
    #[props(extends = button)]
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
    // Order matters there ! Because tw_merge!() will override the first class it finds that overlap another with the last one it finds
    let class = tw_merge!(
        props.base(),
        props.color(),
        props.variant(),
        props.size(),
        props.animation(),
        props.class
    );

    let onclick = move |event| {
        if let Some(oc) = &props.onclick {
            oc.call(event)
        }
    };

    rsx!(
        button {
            ..props.attributes,
            class,
            id: props.id,
            onclick,
            {props.children}
        }
    )
}
