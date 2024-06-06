use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(id, class, children)]
pub fn ButtonGroup() -> Element {
    rsx!(
        div { class: props.class, {props.children} }
    )
}

#[props_component(id, class, children)]
pub fn ButtonGroupItem(
        /// Things like disabled, type,... see [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
        #[props(extends = button)]
        attributes: Vec<Attribute>,
        /// Callback when the button is clicked
        #[props(optional)]
        onclick: Option<EventHandler<MouseEvent>>
        // #[props(default)] color: Color,
        // #[props(default)] size: Size,
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
