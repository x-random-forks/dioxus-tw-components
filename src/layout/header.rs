use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct HeaderLayoutProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for HeaderLayoutProps {
    fn view(self) -> Element {
        let class = HeaderLayoutClass::builder().with_class(self.class);
        rsx!(
            header { class: "{class}", { self.children } }
        )
    }
}

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = r#"sticky w-full top-0 left-0 z-30 border-border border-b backdrop-filter backdrop-blur bg-background/80 overflow-y-hidden flex items-center justify-center"#
)]
pub struct HeaderLayoutClass {}
