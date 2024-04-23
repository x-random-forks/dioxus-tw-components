use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

#[props_component(id, class, children)]
pub fn Form(
    #[props(extends = form)] attributes: Vec<Attribute>,
    #[props(default)] flow: super::FormFlow,
    #[props(optional)] oninput: EventHandler<FormEvent>,
    #[props(optional)] onsubmit: EventHandler<FormEvent>,
) -> Element {
    let class = tw_merge!(props.class);

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
            class: class,
            onsubmit: onsubmit,
            oninput: oninput,
            {props.children}
        }
    )
}
