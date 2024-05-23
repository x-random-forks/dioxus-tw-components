use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(id, class)]
pub fn Input(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional)] value: String,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(optional)] onmounted: Option<EventHandler<Event<MountedData>>>,
    #[props(default)] size: Size,
    #[props(default)] color: Color,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.size(), &props.class);

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
            class,
            id: props.id
        }
    }
}