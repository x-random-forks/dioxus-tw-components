use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(id, class, children)]
pub fn Button(
    #[props(extends = button)] attributes: Vec<Attribute>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] variant: super::ButtonVariant,
    #[props(default)] color: Color,
    #[props(default)] size: Size,
) -> Element {
    // Order matters there ! Because tw_merge!() will override the first class it finds that overlap another with the last one it finds
    let class = tw_merge!(
        props.base(),
        props.color(),
        props.variant(),
        props.size(),
        props.class
    );

    let onclick = move |event| {
        if let Some(oc) = &props.onclick {
            oc.call(event)
        }
    };

    rsx!(
        button { ..props.attributes, class: class, id: props.id, onclick: onclick, {props.children} }
    )
}
