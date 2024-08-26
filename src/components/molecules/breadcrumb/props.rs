use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct BreadcrumbProps {
    #[props(extends = ol, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

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
pub fn Breadcrumb(mut props: BreadcrumbProps) -> Element {
    props.update_class_attribute();

    rsx!(
        ol { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct BreadcrumbItemProps {
    #[props(extends = li, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn BreadcrumbItem(mut props: BreadcrumbItemProps) -> Element {
    props.update_class_attribute();

    rsx!(
        li { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct BreadcrumbSeparatorProps {
    #[props(extends = li, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn BreadcrumbSeparator(mut props: BreadcrumbSeparatorProps) -> Element {
    props.update_class_attribute();

    rsx!(
        li { aria_hidden: "true", ..props.attributes,
            if props.children.is_none() {
                "\u{203A}"
            } else {
                {props.children}
            }
        }
    )
}
