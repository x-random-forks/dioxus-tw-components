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
        div { class: class, id: props.id, {props.children} }
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
        dioxus_free_icons::Icon {
            class: "peer-disabled:cursor-not-allowed peer-disabled:opacity-50 peer-disabled:fill-muted peer-disabled:stroke-muted",
            width: 18,
            height: 18,
            fill: "",
            icon: dioxus_free_icons::icons::fa_regular_icons::FaCircleCheck,
            title: "a checked circle"
        }
    )
}

fn unchecked_circle() -> Element {
    rsx!(dioxus_free_icons::Icon {
        class:
            "peer-disabled:cursor-not-allowed peer-disabled:fill-muted peer-disabled:stroke-muted",
        width: 18,
        height: 18,
        fill: "",
        icon: dioxus_free_icons::icons::fa_regular_icons::FaCircle,
        title: "an hollow circle"
    })
}
