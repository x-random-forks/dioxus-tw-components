use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
// Specifically stylised input type checkbox
// The input use the tailwind peer class, you can use at your advantage to style the children
// eg peer-disabled:font-mute will change children text-color when the input is disabled (Label component already does this by default)
#[props_component(class, id)]
pub fn Toggle(
    #[props(extends = button)] mut attributes: Vec<Attribute>,
    #[props(optional)] checked: Option<bool>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] color: Color,
    #[props(default)] size: Size,
    #[props(default)] animation: Animation,
) -> Element {
    let mut interior_sig = use_signal(|| match props.checked {
        Some(value) => value,
        None => false,
    });

    let state = match interior_sig() {
        true => DataStateAttrValue::Active,
        false => DataStateAttrValue::Inactive,
    };

    let onclick = move |_event| {
        interior_sig.toggle();
        if let Some(oc) = &props.onclick {
            oc.call(_event)
        }
    };

    props.attributes.push(Attribute::new(
        "data-state",
        state.into_value(),
        None,
        false,
    ));

    rsx!(
        button {
            ..props.attributes,
            r#type: "button",
            id: props.id,
            class: props.class,
            onclick
        }
    )
}
