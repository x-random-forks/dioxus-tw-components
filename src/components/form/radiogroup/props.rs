use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

// Struct used to track which RadioItem is currently checked within the RadioGroup
// And differentiate between multiple RadioGroups
struct RadioGroupSignal(String);

#[props_component(class, id, children)]
pub fn RadioGroup(#[props(into)] name: String, #[props(into)] default_value: String) -> Element {
    let class = tw_merge!(props.base(), props.class);

    use_context_provider(|| Signal::new(RadioGroupSignal(props.default_value)));

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

// TO CLEAN/ REFACTOR
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
        (
            true,
            rsx!(
                dioxus_free_icons::Icon {
                    class: "",
                    width: 24,
                    height: 24,
                    icon: dioxus_free_icons::icons::fi_icons::FiCircle,
                    title: "an hollow circle"
                }
            ),
        )
    } else {
        (
            false,
            rsx!(
                dioxus_free_icons::Icon {
                    class: "",
                    width: 24,
                    height: 24,
                    icon: dioxus_free_icons::icons::fi_icons::FiCheckCircle,
                    title: "a checked circle"
                }
            ),
        )
    };

    rsx!(
        label { class: "{props.name}",
            div { class: "flex items-center",
                input {
                    name: "{props.name}",
                    value: "{props.value.clone()}",
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
                    // TODO Move this into another comp
                    div { class: "font-medium peer-disabled:opacity-50 peer-disabled:cursor-not-allowed",
                        {props.children}
                    }
                }
            }
        }
    )
}

// TODO change icons
// fn use_correct_circle_icon(radio_signal: Signal<RadioGroupSignal>, props_value: String) -> Element {
//     rsx!(if radio_signal.read().0 == props_value {
//         dioxus_free_icons::Icon {
//             class: "",
//             width: 24,
//             height: 24,
//             icon: dioxus_free_icons::icons::fi_icons::FiCircle,
//             title: "an hollow circle",
//         }
//     } else {
//         dioxus_free_icons::Icon {
//             class: "",
//             width: 24,
//             height: 24,
//             icon: dioxus_free_icons::icons::fi_icons::FiCheckCircle,
//             title: "a checked circle",
//         }
//     })
// }
