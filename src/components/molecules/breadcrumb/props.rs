use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
/// Usage:
/// ```ignore
/// Breadcrumb {
///     BreadcrumbItem { "Home" },
///     BreadcrumbSeparator {},
///     BreadcrumbItem { "Library" },
///     BreadcrumbSeparator {},
///     BreadcrumbItem { "Data" },
/// }
/// ```
#[props_component(class, id, children)]
pub fn Breadcrumb(#[props(default)] separator: bool) -> Element {
    rsx!(
        ol { class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn BreadcrumbItem() -> Element {
    rsx!(
        li { class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn BreadcrumbSeparator() -> Element {
    rsx!(
        li { class: props.class, aria_hidden: "true", id: props.id,
            if props.children == None {
                "\u{203A}"
            } else {
                {props.children}
            }
        }
    )
}