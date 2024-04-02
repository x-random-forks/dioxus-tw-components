use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct NavbarProps {
    #[props(default)]
    left_part: Element,
    #[props(default)]
    right_part: Element,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for NavbarProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<NavbarProps>::BaseClass, self.class);

        // TODO Move this ?
        let left_part_class = "flex space-x-2 items-center ml-6";
        let right_part_class = "flex flex-1 items-center justify-end space-x-2";
        rsx!(
            nav { class: "{class}",
                div { class: "{left_part_class}", {self.left_part} }
                div { class: "{right_part_class}", {self.right_part} }
            }
        )
    }
}
