use crate::{components::atom::icon::*, components::form::radiogroup::props::style::IconSvg};
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

    let (svg, checked) = if radio_context.read().0 == props.value {
        (IconSvg::CircleInnerCircle, true)
    } else {
        (IconSvg::HollowCircle, false)
    };

    rsx!(
        label { class: "{props.name}",
            div { class: "flex items-center",
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
                    div { class: "size-4 peer-disabled:cursor-not-allowed", Icon { svg: svg } }
                } else {

                    div { class: "size-4 peer-disabled:cursor-not-allowed", Icon { svg: svg } }
                    // TODO Move this into another comp
                    div { class: "font-medium peer-disabled:opacity-50 peer-disabled:cursor-not-allowed",
                        {props.children}
                    }
                }
            }
        }
    )
}
