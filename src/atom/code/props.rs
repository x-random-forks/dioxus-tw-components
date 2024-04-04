use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct CodeProps {
    #[props(default)]
    class: String,
}

impl Component for CodeProps {
    fn view(self) -> Element {
        // let class = CodeClass::builder().with_class(self.class);
        let class = "";
        rsx!(
            pre { code { class: "{class}" } }
        )
    }
}
