use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct DocsLayoutProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for DocsLayoutProps {
    fn view(self) -> Element {
        let class = DocsLayoutClass::builder().with_class(self.class);
        rsx!(
            div { class: "{class}", { self.children } }
        )
    }
}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"w-full"#)]
pub struct DocsLayoutClass {}
