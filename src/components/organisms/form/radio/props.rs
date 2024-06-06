use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(class, children, id)]
pub fn Radio(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional, default = false)] checked: bool,
    #[props(optional)] oninput: EventHandler<FormEvent>,
    #[props(default)] color: Color,
) -> Element {
    rsx!(
        input {
            ..props.attributes,
            id: &*props.id,
            r#type: "radio",
            checked: props.checked,
            class: props.class,
            oninput: move |e| {
                props.oninput.call(e);
            }
        }
    )
}
