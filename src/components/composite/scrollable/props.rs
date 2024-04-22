use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(class, children, id)]
pub fn Scrollable(#[props(default = Orientation::Vertical)] orientation: Orientation) -> Element {
    let class = tw_merge!(props.base(), props.orientation(), props.class);

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
