use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

#[props_component(class, children, id)]
pub fn Docs() -> Element {
    let class = tw_merge!("mx-auto w-full border", props.class);

    rsx!(
        div { class: class, id: props.id, { props.children } }
    )
}
