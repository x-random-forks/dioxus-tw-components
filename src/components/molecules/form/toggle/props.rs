use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

// Specifically stylised input type checkbox
// The input use the tailwind peer class, you can use at your advantage to style the children
// eg peer-disabled:font-mute will change children text-color when the input is disabled (Label component already does this by default)
#[props_component(class, id)]
pub fn Toggle(
    #[props(extends = button)] mut attributes: Vec<Attribute>,
    #[props(default = false)] active: bool,
    #[props(optional)] onclick: Option<EventHandler<FormEvent>>,
    #[props(default)] color: Color,
    #[props(default)] size: Size,
    #[props(default)] animation: Animation,
) -> Element {
    let class = tw_merge!(
        props.base(),
        props.color(),
        props.size(),
        props.animation(),
        props.class
    );

    let mut state = use_signal(|| match props.active {
        true => DataStateAttrValue::Active,
        false => DataStateAttrValue::Inactive,
    });

    let onclick = move |_event| {
        state.write().toggle();
    };

    props.attributes.push(Attribute::new(
        "data-state",
        state.read().into_value(),
        None,
        false,
    ));

    rsx!(
        button {
            ..props.attributes,
            id: props.id,
            class,
            onclick: onclick
        }
    )
}
