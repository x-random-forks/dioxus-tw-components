use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct SeparatorProps {
    #[props(default = false)]
    vertical: bool,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for SeparatorProps {
    fn view(self) -> Element {
        let class = SeparatorClass::builder()
            .vertical(self.vertical.into())
            .with_class(self.class);

        rsx!( div { class: "{class}" } )
    }
}
