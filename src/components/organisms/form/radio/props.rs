use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(class, children, id)]
pub fn Radio(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional, default = false)] checked: bool,
    #[props(optional)] oninput: EventHandler<FormEvent>,
    #[props(default)] color: Color,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.class);

    rsx!(
        input {
            ..props.attributes,
            id: &*props.id,
            r#type: "radio",
            checked: props.checked,
            class,
            oninput: move |e| {
                props.oninput.call(e);
            },
        }
    )
}
