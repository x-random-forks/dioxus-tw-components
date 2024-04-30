use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(id, class, children)]
pub fn Form(
    #[props(extends = form)] attributes: Vec<Attribute>,
    #[props(optional)] oninput: EventHandler<FormEvent>,
    #[props(optional)] onsubmit: EventHandler<FormEvent>,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    let oninput = move |event: FormEvent| {
        props.oninput.call(event);
    };

    let onsubmit = move |event: FormEvent| {
        props.onsubmit.call(event);
    };

    rsx!(
        form {
            ..props.attributes,
            id: props.id,
            class,
            onsubmit,
            oninput,
            {props.children}
        }
    )
}

#[props_component(class, children)]
pub fn FormHeader() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class, { props.children } }
    )
}

#[props_component(class, children)]
pub fn FormFooter() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class, { props.children } }
    )
}
