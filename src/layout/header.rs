use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

#[props_component(class, children, id)]
pub fn HeaderLayout() -> Element {
    let class = tw_merge!("sticky w-full top-0 left-0 z-30 border-border border-b backdrop-filter backdrop-blur bg-background/80 overflow-y-hidden flex items-center justify-center", props.class);

    rsx!(
        header { class, id: props.id, { props.children } }
    )
}
