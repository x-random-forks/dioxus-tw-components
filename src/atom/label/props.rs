use self::styling::Color;
use crate::{styling::BaseClass, *};
use component_derive::Component;

pub use Color::*;

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
        let class = class!(BaseClass::<LabelProps>::Default, self.color);
        rsx!(
            label { class: "{class}", r#for: "{self.r#for}", { self.children } }
        )
    }
}
