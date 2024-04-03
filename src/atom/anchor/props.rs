use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct AnchorProps {
    #[props(default = "#".to_string())]
    href: String,
    #[props(default)]
    id: String,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for AnchorProps {
    fn view(self) -> Element {
        let class = AnchorClass::builder().with_class(self.class);
        rsx!( a { class: "{class}" } )
    }
}
