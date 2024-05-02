use crate::{atoms::ButtonVariant, attributes::Color, components::atoms::Button};
use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

// TODO : This is more of an example, probably need to use it before refining implementation
#[props_component(class, children)]
pub fn FormList(#[props(default)] list_fields: Vec<Element>) -> Element {
    let class = tw_merge!(props.base(), &props.class);

    let mut size_to_render = use_signal(|| 1);

    let rendered_group_vec = props
        .list_fields
        .iter()
        .take(size_to_render())
        .map(|x| rsx!(
            { x }.clone(),
            div { class: "h-4" }
        ));

    let vec_size = props.list_fields.len();

    let button_closure_plus = move |_| {
        if size_to_render() < vec_size {
            size_to_render += 1;
        }
    };

    let button_closure_minus = move |_| {
        if size_to_render() > 1 {
            size_to_render -= 1;
        }
    };

    let button_class = "size-10 inline-flex items-center justify-center";

    rsx!(
        div { class,
            h4 { class: "h4", {props.children} }
            div { class: "sticky top-0",
                Button {
                    class: button_class,
                    variant: ButtonVariant::Ghost,
                    onclick: button_closure_plus,
                    r#type: "button",
                    "+"
                }
                Button {
                    class: button_class,
                    variant: ButtonVariant::Ghost,
                    onclick: button_closure_minus,
                    r#type: "button",
                    "-"
                }
            }
            { rendered_group_vec }
        }
    )
}