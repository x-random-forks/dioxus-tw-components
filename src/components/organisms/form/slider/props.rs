use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(id, class)]
pub fn Slider(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional)] value: String,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(optional)] onmounted: Option<EventHandler<Event<MountedData>>>,
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

    rsx!(
        input {
            ..props.attributes,
            r#type: "range",
            value: props.value,
            class: props.class,
            id: props.id,
            onmounted,
            oninput
        }
    )
}

#[props_component(id, class)]
pub fn SliderTicks(
    #[props(optional, default = 10)] step: i64,
    #[props(optional, default = 0)] min: i64,
    #[props(optional, default = 100)] max: i64,
) -> Element {
    rsx!(
        datalist { class: props.class, id: props.id,
            for i in props.min..props.max {
                if i % props.step == 0 {
                    option { value: i }
                }
            }
            option { value: props.max }
        }
    )
}

#[props_component(id, class)]
pub fn SliderLabel(
    #[props(optional, default = 0)] value: i64,
    #[props(optional, default = 100)] max: i64,
) -> Element {
    rsx!(
        div { class: props.class,
            {props.value.to_string()},
            " / "
            {props.max.to_string()}
        }
    )
}

impl Named for SliderProps {
    const NAME: &'static str = "Slider";
}