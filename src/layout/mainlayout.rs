use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

#[props_component(class, children, id)]
pub fn MainLayout() -> Element {
    let class = tw_merge!("container flex-1 items-start md:grid md:grid-cols-[220px_minmax(0,1fr)] md:gap-6 lg:grid-cols-[240px_minmax(0,1fr)] lg:gap-10", props.class);

    rsx!(
        div { class: class, id: props.id, { props.children } }
    )
}
