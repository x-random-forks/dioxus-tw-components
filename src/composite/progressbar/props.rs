use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(class, children, id)]
pub fn ProgressTrack(
    #[props(default = Color::Primary)] color: Color,
    #[props(default)] size: Size,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.size(), props.class);

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, children, id)]
pub fn ProgressBar(#[props(default = 50)] progress: u8, #[props(default)] color: Color) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.class);

    rsx!(
        div { class: class, style: "width: {props.progress}%", id: props.id,
            div { {props.children} }
        }
    )
}

#[props_component(class, children, id)]
pub fn ProgressLabel(
    #[props(default = 50)] progress: u8,
    #[props(default = true)] show_percentage: bool,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        span { class: class, {props.progress.to_string()}, if props.show_percentage { "%" } }
    )
}
