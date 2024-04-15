use crate::{atom::icon::*, composite::radiogroup::props::style::IconSvg};
use dioxus::prelude::*;
use myderive::props_component;
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

// TO CLEAN
#[props_component(class, id, children)]
pub fn RadioItem(
    // Corresponds to the name of the RadioGroup
    #[props(default)] name: String,
    // What will be sent as name:value
    #[props(default)] value: String,
    // Applies to the whole RadioGroup, if true, the form will not be submitted if no RadioItem is checked
    #[props(default = false)] required: bool,
    // Disable the radio button
    #[props(default = false)] disabled: bool,

    #[props(optional)] oninput: EventHandler<FormEvent>,
) -> Element {
    let mut radio_context = consume_context::<Signal<RadioGroupSignal>>();

    let svg = if radio_context.read().0 == props.value {
        IconSvg::CircleInnerCircle
    } else {
        IconSvg::HollowCircle
    };

    let checked = if radio_context.read().0 == props.value {
        true
    } else {
        false
    };

    rsx!(
        label { class: "{props.name}",
            div { class: "flex items-center",
                input {
                    name: "{props.name}",
                    value: "{props.value}",
                    checked: "{checked}",
                    required: "{props.required}",
                    disabled: "{props.disabled}",
                    r#type: "radio",
                    oninput: move |e| {
                        radio_context.set(RadioGroupSignal(props.value.clone()));
                        props.oninput.call(e);
                    },
                    class: "hidden peer"
                }
                div { class: "size-4 peer-disabled:cursor-not-allowed", Icon { svg: svg } }
                // TODO Move this into another comp
                div { class: "font-medium peer-disabled:opacity-50 peer-disabled:cursor-not-allowed",
                    {props.children}
                }
            }
        }
    )
}

// TODO Refactor using macros
// #[derive(PartialEq, Props, Clone, Component)]
// pub struct RadioItemProps {
//     // Corresponds to the name of the RadioGroup
//     #[props(default)]
//     name: String,
//     // What will be sent as name:value
//     #[props(default)]
//     value: String,
//     // Applies to the whole RadioGroup, if true, the form will not be submitted if no RadioItem is checked
//     #[props(default = false)]
//     required: bool,
//     // Disable the radio button
//     #[props(default = false)]
//     disabled: bool,

//     #[props(optional)]
//     oninput: EventHandler<FormEvent>,

//     children: Element,

//     // Styling
//     #[props(default)]
//     class: String,
// }

// impl Component for RadioItemProps {
//     fn view(self) -> Element {
//         let mut radio_context = consume_context::<Signal<RadioGroupSignal>>();

//         // TODO Should do both at the same time
//         let svg = if radio_context.read().0 == self.value {
//             IconSvg::CircleInnerCircle
//         } else {
//             IconSvg::HollowCircle
//         };

//         let checked = if radio_context.read().0 == self.value {
//             true
//         } else {
//             false
//         };

//         rsx!(
//             label { class: "{self.name}",
//                 div { class: "flex items-center",
//                     input {
//                         name: "{self.name}",
//                         value: "{self.value}",
//                         checked: "{checked}",
//                         required: "{self.required}",
//                         disabled: "{self.disabled}",
//                         r#type: "radio",
//                         oninput: move |e| {
//                             radio_context.set(RadioGroupSignal(self.value.clone()));
//                             self.oninput.call(e);
//                         },
//                         class: "hidden peer"
//                     }
//                     div { class: "size-4 peer-disabled:cursor-not-allowed", Icon { svg: svg } }
//                     // TODO Move this into another comp
//                     div { class: "font-medium peer-disabled:opacity-50 peer-disabled:cursor-not-allowed",
//                         {self.children}
//                     }
//                 }
//             }
//         )
//     }
// }
