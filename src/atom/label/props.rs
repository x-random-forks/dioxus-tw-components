use self::styling::{BaseClass, Color};
use crate::*;
use component_derive::Component;

pub use Color::{Accent, Primary, Secondary};

#[derive(PartialEq, Props, Clone, Component)]
pub struct LabelProps {
    #[props(default)]
    class: String,
    // Represent the unique id in the DOM
    #[props(default)]
    r#for: String,
    children: Element,
    // Styling
    #[props(default)]
    color: Color<LabelProps>,
}

impl Component for LabelProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<LabelProps>::BaseClass, self.color);
        rsx!(
            label { class: "{class}", r#for: "{self.r#for}", { self.children } }
        )
    }
}
