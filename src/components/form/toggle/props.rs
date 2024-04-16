use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

// TODO / REVIEW : Same as checkbox, split this into more components ?

// Specifically stylised input type checkbox
// The input use the tailwind peer class, you can use at your advantage to style the children
// eg peer-disabled:font-mute will change children text-color when the input is disabled (Label component already does this by default)
#[props_component(class, id, children)]
pub fn Toggle(
    #[props(extends = input)] attributes: Vec<Attribute>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(default)] color: Color,
    #[props(default)] size: Size,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.size(), props.class);

    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    rsx!(
        // Label that wraps the input and the toggle switch so the user can click on the switch or the children to interact with the input
        label { class: "peer flex items-center cursor-pointer gap-x-2",
            input {
                r#type: "checkbox",
                ..props.attributes,
                // Set this input to be hidden except for screen readers
                // We a custom visual toggle so we don't need the default input
                class: "sr-only peer",
                oninput: oninput
            }
            // Div that renders the toggle switch
            div { class: "{class}" }
            div { class: "peer-disabled:opacity-50", {props.children} }
        }
    )
}
