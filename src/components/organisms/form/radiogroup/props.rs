use core::panic;

use dioxus::{dioxus_core::AttributeValue, prelude::*};
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

// Struct used to track which RadioItem is currently checked within the RadioGroup
// And differentiate between multiple RadioGroups
struct RadioGroupSignal {
    default_value: String,
}

impl RadioGroupSignal {
    fn new(default_value: String) -> Self {
        Self { default_value }
    }

    fn set(&mut self, value: String) {
        self.default_value = value;
    }

    fn get_correct_circle(&self, value: String) -> Element {
        if self.default_value == value {
            rsx!(
                svg {
                    class: "fill-current stroke-current",
                    width: 18,
                    height: 18,
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 512 512",
                    path { d: "M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-111 111-47-47c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l64 64c9.4 9.4 24.6 9.4 33.9 0L369 209z" }
                }
            )
        } else {
            rsx!(
                svg {
                    class: "fill-current stroke-current",
                    width: 18,
                    height: 18,
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 512 512",
                    path { d: "M464 256A208 208 0 1 0 48 256a208 208 0 1 0 416 0zM0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256z" }
                }
            )
        }
    }
}

#[props_component(class, id, children)]
pub fn RadioGroup(
    #[props(into)] name: String,
    #[props(default)]
    #[props(into)]
    default_value: String,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    use_context_provider(|| Signal::new(RadioGroupSignal::new(props.default_value)));

    rsx!(
        div { class, id: props.id, {props.children} }
    )
}

// TO CLEAN/ REFACTOR : probably while building a real form, will probably have to use a button or something similar instead of input tag
#[props_component(class, children, id)]
pub fn RadioItem(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional)] oninput: EventHandler<FormEvent>,
) -> Element {
    let mut state = consume_context::<Signal<RadioGroupSignal>>();

    let value = props
        .attributes
        .iter()
        .find(|a| a.name == "value")
        .and_then(|attr| {
            let AttributeValue::Text(value) = attr.value.clone() else {
                return None;
            };
            Some(value)
        })
        .unwrap_or_default();

    let value = use_signal(|| value);

    let circle = state.read().get_correct_circle(value());

    rsx!(
        label {
            input {
                ..props.attributes,
                id: &*props.id,
                r#type: "radio",
                oninput: move |e| {
                    state.write().set(value());
                    props.oninput.call(e);
                },
                class: "peer hidden"
            }
            div {
                id: &*props.id,
                class: "size-4 peer-disabled:cursor-not-allowed",
                {circle}
            }
        }
    )
}
