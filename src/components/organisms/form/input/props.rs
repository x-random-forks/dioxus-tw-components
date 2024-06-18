use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(id, class)]
pub fn Input(
    #[props(extends = input, extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(optional)] value: String,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(optional)] onmounted: Option<EventHandler<Event<MountedData>>>,
    #[props(default)] size: Size,
    #[props(default)] color: Color,
) -> Element {
    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    let onmounted = move |event: Event<MountedData>| {
        if let Some(oc) = &props.onmounted {
            oc.call(event)
        }
    };

    rsx! {
        input {
            ..props.attributes,
            value: props.value,
            onmounted,
            oninput,
            class: props.class,
            id: props.id
        }
    }
}