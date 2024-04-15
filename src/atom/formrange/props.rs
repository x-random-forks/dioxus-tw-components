use dioxus::prelude::*;
use myderive::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(id, class)]
pub fn FormRange(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    rsx!(
        input {
            ..props.attributes,
            r#type: "range",
            class: class,
            id: props.id,
            oninput: oninput
        }
    )
}
