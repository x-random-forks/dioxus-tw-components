use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct ScrollableProps {
    #[props(default = false)]
    horizontal: bool,
    children: Element,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for ScrollableProps {
    fn view(self) -> Element {
        let class = super::ScrollableClass::builder()
            .horizontal(self.horizontal.into())
            .with_class(self.class);

        rsx!(
            div { class: "{class}", {self.children} }
        )
    }
}
