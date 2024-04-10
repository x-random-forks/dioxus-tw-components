use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props_no_children!(NavbarProps {
    #[props(default)]
    left_part: Element,
    #[props(default)]
    right_part: Element,
});

// TODO: Split this into multiple components
impl Component for NavbarProps {
    fn view(self) -> Element {
        let class = super::NavbarClass::builder().with_class(self.class);

        // TODO Move this
        let left_part_class = "flex space-x-2 items-center ml-6";
        let right_part_class = "flex flex-1 items-center justify-end space-x-2 mr-6";
        rsx!(
            nav { class: "{class}",
                div { class: "{left_part_class}", {self.left_part} }
                div { class: "{right_part_class}", {self.right_part} }
            }
        )
    }
}
