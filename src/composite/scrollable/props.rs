use dioxus::prelude::*;
use myderive::props_component;
use tailwind_fuse::*;

use crate::types::*;

#[props_component(class, children, id)]
pub fn Scrollable(#[props(default = false)] horizontal: bool) -> Element {
    let class = tw_merge!(props.base(), props.variant(), props.class);

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

// #[derive(PartialEq, Props, Clone, Component)]
// pub struct ScrollableProps {
//     #[props(default = false)]
//     horizontal: bool,
//     children: Element,

//     // Styling
//     #[props(default)]
//     class: String,
// }

// impl Component for ScrollableProps {
//     fn view(self) -> Element {
//         let class = super::ScrollableClass::builder()
//             .horizontal(self.horizontal.into())
//             .with_class(self.class);

//         rsx!(
//             div { class: "{class}", {self.children} }
//         )
//     }
// }
