use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct ScrollableProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for ScrollableProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<ScrollableProps>::BaseClass, self.class);

        rsx!(
            div { class: "{class}", {self.children} }
        )
    }
}
