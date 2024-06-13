use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(class, children, id)]
pub fn ProgressBar(
    #[props(default = Color::Primary)] color: Color,
    #[props(default)] size: Size,
) -> Element {
    rsx!(
        div { class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, children, id)]
pub fn ProgressBarInner(#[props(default = 50)] progress: u8, #[props(default)] color: Color) -> Element {
    let progress = if props.progress > 100 {
        100
    } else {
        props.progress
    };

    rsx!(
        div { class: props.class, style: "width: {progress}%", id: props.id,
            div { {props.children} }
        }
    )
}

#[props_component(class, children, id)]
pub fn ProgressLabel(
    #[props(default = 50)] progress: u8,
    #[props(default = true)] show_percentage: bool,
) -> Element {
    rsx!(
        span { class: props.class,
            {props.progress.to_string()},
            if props.show_percentage {
                "%"
            }
        }
    )
}
