use component_derive::Component;

use crate::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct LabelProps {
    // Represent the unique id in the DOM
    #[props(default)]
    r#for: String,
    children: Element,
}

impl Component for LabelProps {
    fn view(self) -> Element {
        rsx!(label {
            class: "text-foreground font-bold",
            r#for: "{self.r#for}",
            { self.children }}
        )
    }
}
