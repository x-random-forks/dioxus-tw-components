use dioxus::prelude::*;
use props_component_macro::props_component;
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
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        ol { class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn BreadcrumbItem() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        li { class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn BreadcrumbSeparator() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        li { class: class, aria_hidden: "true", id: props.id,
            if props.children == None {
                "\u{203A}"
            } else {
                {props.children}
            }
        }
    )
}
