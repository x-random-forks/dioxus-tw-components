use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct LabelProps {
    // Represent the unique id in the DOM
    #[props(default)]
    r#for: String,

    children: Element,

    // Styling
    #[props(default)]
    color: LabelColor,
    #[props(default)]
    class: String,
}

// REVIEW / TODO : While probably get rid of this class in the end and just add a .label class in input.css
impl Component for LabelProps {
    fn view(self) -> Element {
        let class = LabelClass::builder()
            .color(self.color)
            .with_class(self.class);

        rsx!(
            label { class: "{class}", r#for: "{self.r#for}", { self.children } }
        )
    }
}
