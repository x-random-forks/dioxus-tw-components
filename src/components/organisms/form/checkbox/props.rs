use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(id, class)]
pub fn Checkbox(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional, default = false)] checked: bool,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(default)] color: Color,
) -> Element {
    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    rsx!(
        input {
            ..props.attributes,
            r#type: "checkbox",
            checked: props.checked,
            class: props.class,
            oninput: oninput,
            id: props.id
        }
    )
}
