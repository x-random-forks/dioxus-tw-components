use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct SeparatorProps {
    #[props(default = false)]
    vertical: bool,
    // Use to set custom tailwind classes, generally used for margin and padding between components its is separating
    #[props(default)]
    class: String,
}

impl Component for SeparatorProps {
    fn view(self) -> Element {
        let orientation_class = get_orientation_class(self.vertical);

        let class = class!(
            BaseClass::<SeparatorProps>::BaseClass,
            orientation_class,
            self.class
        );
        rsx!( div { class: "{class}" } )
    }
}

// TODO Move this to style.rs
fn get_orientation_class(vertical: bool) -> &'static str {
    if vertical {
        "h-full w-[1px]"
    } else {
        "w-full h-[1px]"
    }
}
