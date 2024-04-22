use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(id, class)]
pub fn Checkbox(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(default = Color::Primary)] color: Color,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.class);

    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    rsx!(
        input {
            ..props.attributes,
            r#type: "checkbox",
            class: class,
            oninput: oninput,
            id: props.id
        }
    )
}
