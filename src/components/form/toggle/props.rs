use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

// Specifically stylised input type checkbox
// The input use the tailwind peer class, you can use at your advantage to style the children
// eg peer-disabled:font-mute will change children text-color when the input is disabled (Label component already does this by default)
#[props_component(class, id)]
pub fn Toggle(
    #[props(extends = button)] attributes: Vec<Attribute>,
    #[props(default = false)] checked: bool,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(default)] color: Color,
    #[props(default)] size: Size,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.size(), props.class);

    let mut state = use_signal(|| match props.checked {
        true => "on".to_string(),
        false => "off".to_string(),
    });

    let onclick = move |_event| {
        if state() == "off" {
            state.set("on".to_string());
        } else {
            state.set("off".to_string());
        }
    };

    rsx!(
        button {
            "data-state": state(),
            ..props.attributes,
            id: props.id,
            class: class,
            onclick: onclick
        }
    )
}
