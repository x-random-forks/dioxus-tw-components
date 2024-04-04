use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct MainLayoutProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for MainLayoutProps {
    fn view(self) -> Element {
        let class = MainLayoutClass::builder().with_class(self.class);
        rsx!(
            div { class: "{class}", { self.children } }
        )
    }
}

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = r#"container flex-1 items-start md:grid md:grid-cols-[220px_minmax(0,1fr)] md:gap-6 lg:grid-cols-[240px_minmax(0,1fr)] lg:gap-10"#
)]
pub struct MainLayoutClass {}
