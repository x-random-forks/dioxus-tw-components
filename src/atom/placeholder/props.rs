use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq, Component)]
pub struct PlaceholderProps {
    // Styling
    #[props(default)]
    radius: PlaceholderRadius,
    #[props(default = true)]
    animation: bool,
    #[props(default)]
    class: String,
}

impl Component for PlaceholderProps {
    fn view(self) -> Element {
        let class = PlaceholderClass::builder()
            .radius(self.radius)
            .animation(self.animation.into())
            .with_class(self.class);

        rsx!( div { class: "{class}" } )
    }
}
