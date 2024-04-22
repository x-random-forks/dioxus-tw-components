use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(class, id, children)]
pub fn Table(#[props(extends = table)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        table { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableHeader(#[props(extends = thead)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        thead { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableBody(#[props(extends = tbody)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        tbody { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableFooter(#[props(extends = tfoot)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        tfoot { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableHead(#[props(extends = th)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        th { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableRow(#[props(extends = tr)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        tr { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableCell(#[props(extends = td)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        td { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TableCaption(#[props(extends = caption)] attributes: Vec<Attribute>) -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        caption { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}
