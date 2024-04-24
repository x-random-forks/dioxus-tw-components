use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

// Struct used to track which RadioItem is currently checked within the RadioGroup
// And differentiate between multiple RadioGroups
struct RadioGroupSignal(String);

#[props_component(class, id, children)]
pub fn RadioGroup(
    #[props(into)] name: String,
    #[props(default)]
    #[props(into)]
    default_value: String,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    use_context_provider(|| Signal::new(RadioGroupSignal(props.default_value)));

    rsx!(
        div { class, id: props.id, {props.children} }
    )
}

// TO CLEAN/ REFACTOR : probably while building a real form, will probably have to use a button or something similar instead of input tag
#[props_component(class, id, children)]
pub fn RadioItem(
    // Corresponds to the name of the RadioGroup
    #[props(default)] name: String,
    // What will be sent as name:value
    #[props(default)] value: String,
    // Disable the radio button
    #[props(default = false)] disabled: bool,

    #[props(optional)] oninput: EventHandler<FormEvent>,

    #[props(default)] side: Side,
) -> Element {
    let mut radio_context = consume_context::<Signal<RadioGroupSignal>>();

    let (checked, circle) = if radio_context.read().0 == props.value {
        (true, checked_circle())
    } else {
        (false, unchecked_circle())
    };

    rsx!(
        label { class: "{props.name}",
            div { class: "flex items-center space-x-2",
                input {
                    name: "{props.name}",
                    value: "{props.value}",
                    checked: "{checked}",
                    disabled: "{props.disabled}",
                    r#type: "radio",
                    oninput: move |e| {
                        radio_context.set(RadioGroupSignal(props.value.clone()));
                        props.oninput.call(e);
                    },
                    class: "hidden peer"
                }
                if props.side == Side::Left {
                    div { class: "font-medium peer-disabled:opacity-50 peer-disabled:cursor-not-allowed",
                        {props.children}
                    }
                    div { class: "size-4 peer-disabled:cursor-not-allowed", {circle} }
                } else {
                    div { class: "size-4 peer-disabled:cursor-not-allowed", {circle} }
                    div { class: "font-medium peer-disabled:opacity-50 peer-disabled:cursor-not-allowed",
                        {props.children}
                    }
                }
            }
        }
    )
}

fn checked_circle() -> Element {
    rsx!(
        svg {
            width: 18,
            height: 18,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 512 512",
            path {
                d: "M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-111 111-47-47c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l64 64c9.4 9.4 24.6 9.4 33.9 0L369 209z"
            }
        }
    )
}

fn unchecked_circle() -> Element {
    rsx!(
        svg {
            width: 18,
            height: 18,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 512 512",
            path {
                d: "M464 256A208 208 0 1 0 48 256a208 208 0 1 0 416 0zM0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256z"
            }
        }
    )
}
