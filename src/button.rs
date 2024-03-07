use component_derive::Component;

use crate::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct ButtonProps {
    score: Option<i32>,
    onclick: EventHandler<MouseEvent>,
    children: Element,
}

// mandatory
impl Component for ButtonProps {
    fn view(self) -> Element {
        rsx! {
            button {
                class: "bg-blue-500 text-white font-bold py-2 px-4 rounded",
                onclick: move |e| { self.onclick.call(e) },
                { self.children }
            }
        }
    }
}
