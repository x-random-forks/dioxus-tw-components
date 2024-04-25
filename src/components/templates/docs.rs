use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

#[props_component(class, children, id)]
pub fn DocsTemplate() -> Element {
    let class = tw_merge!("mx-auto w-full border", props.class);

    rsx!(
        div { class, id: props.id, { props.children } }
    )
}
