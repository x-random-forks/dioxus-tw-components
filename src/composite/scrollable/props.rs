use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct ScrollableProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for ScrollableProps {
    fn view(self) -> Element {
        let class = ScrollableClass::builder().with_class(self.class);

        rsx!(
            div { class: "{class}", {self.children} }
        )
    }
}
