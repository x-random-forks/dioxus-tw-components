use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[props_component(class, id, children)]
pub fn Table(#[props(extends = table)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        table { ..props.attributes, class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableHeader(#[props(extends = thead)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        thead { ..props.attributes, class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableBody(#[props(extends = tbody)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        tbody { ..props.attributes, class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableFooter(#[props(extends = tfoot)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        tfoot { ..props.attributes, class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableHead(
    #[props(extends = th)] attributes: Vec<Attribute>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let onclick = move |event| {
        if let Some(oc) = &props.onclick {
            oc.call(event)
        }
    };

    rsx!(
        th {
            ..props.attributes,
            class: props.class,
            id: props.id,
            onclick,
            {props.children}
        }
    )
}

#[props_component(class, id, children)]
pub fn TableRow(#[props(extends = tr)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        tr { ..props.attributes, class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableCell(#[props(extends = td)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        td { ..props.attributes, class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableCaption(#[props(extends = caption)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        caption { ..props.attributes, class: props.class, id: props.id, {props.children} }
    )
}
