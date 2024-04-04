use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct BreadcrumbProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for BreadcrumbProps {
    fn view(self) -> Element {
        let class = BreadcrumbClass::builder().with_class(self.class);
        rsx!(
            ol { class: "{class}", { self.children } }
        )
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct BreadcrumbItemProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for BreadcrumbItemProps {
    fn view(self) -> Element {
        let class = BreadcrumbSeparatorClass::builder().with_class(self.class);
        rsx!(
            li { class: "{class}", { self.children } }
        )
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct BreadcrumbSeparatorProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for BreadcrumbSeparatorProps {
    fn view(self) -> Element {
        let class = BreadcrumbSeparatorClass::builder().with_class(self.class);
        rsx!(
            li { class: "{class}", aria_hidden: "true",
                if self.children == None {
                    "\u{203A}"
                } else {
                    { self.children }
                }
            }
        )
    }
}
