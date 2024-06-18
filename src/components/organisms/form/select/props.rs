use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(class, id, children)]
pub fn SelectGroup(
    #[props(extends = select, extends = GlobalAttributes)] attributes: Vec<Attribute>,

    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    rsx!(
        select {
            ..props.attributes,
            class: props.class,
            id: props.id,
            oninput: oninput,
            {props.children}
        }
    )
}

#[props_component(class, id, children)]
pub fn SelectPlaceholder() -> Element {
    rsx!(
        option {
            disabled: true,
            selected: true,
            value: r#"{""}"#,
            class: props.class,
            id: props.id,
            {props.children}
        }
    )
}

#[props_component(class, id)]
pub fn SelectLabel(#[props(extends = optgroup, extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        optgroup { ..props.attributes, class: props.class, id: props.id }
    )
}

#[props_component(class, id, children)]
pub fn SelectItem(
    #[props(extends = option, extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(optional, default = None)] selected: Option<bool>,
) -> Element {
    if let Some(selected) = props.selected {
        rsx!(
            option {
                ..props.attributes,
                class: props.class,
                selected,
                id: props.id,
                {props.children}
            }
        )
    } else {
        rsx!(
            option { ..props.attributes, class: props.class, id: props.id, {props.children} }
        )
    }
}