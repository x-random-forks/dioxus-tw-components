use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct CodeProps {
    #[props(default)]
    class: String,
}

// REVIEW / TODO: How to do it ?
// See https://discord.com/channels/899851952891002890/1224454726771544114/1224454726771544114
// Ewan and Jkelley tell how they do it on dioxus website
impl Component for CodeProps {
    fn view(self) -> Element {
        // let class = CodeClass::builder().with_class(self.class);
        let class = "";
        rsx!(
            pre { code { class: "{class}" } }
        )
    }
}
