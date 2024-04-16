use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

#[props_component(id, class, children)]
pub fn Form(
    #[props(extends = form)] attributes: Vec<Attribute>,
    #[props(default)] flow: super::FormFlow,
) -> Element {
    let class = tw_merge!(props.class);

    rsx!(
        form { ..props.attributes, id: props.id, class: class, {props.children} }
    )
}
