use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct SeparatorProps {
    #[props(default = false)]
    vertical: bool,
    // Use to set custom tailwind classes, generally used for margin and padding between components its is separating
    #[props(default)]
    class: String,
}

impl Component for SeparatorProps {
    fn view(self) -> Element {
        let class = SeparatorClass::builder()
            .vertical(match self.vertical {
                true => SeparatorOrientation::Vertical,
                false => SeparatorOrientation::Horizontal,
            })
            .with_class(self.class);

        rsx!( div { class: "{class}" } )
    }
}
