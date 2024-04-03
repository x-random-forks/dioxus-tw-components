use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq, Component)]
pub struct DropdownProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for DropdownProps {
    fn view(self) -> Element {
        let class = DropdownClass::builder().with_class(self.class);
        rsx!(
            div { class: "{class}", {self.children} }
        )
    }
}

#[derive(Props, Clone, PartialEq, Component)]
pub struct DropdownToggleProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for DropdownToggleProps {
    fn view(self) -> Element {
        let class = DropdownToggleClass::builder().with_class(self.class);
        rsx!(
            div { class: "{class}", {self.children} }
        )
    }
}

#[derive(Props, Clone, PartialEq, Component)]
pub struct DropdownContentProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for DropdownContentProps {
    fn view(self) -> Element {
        let class = DropdownContentClass::builder().with_class(self.class);
        rsx!(
            div { class: "{class}", {self.children} }
        )
    }
}
