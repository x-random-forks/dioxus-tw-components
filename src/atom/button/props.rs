use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq, Component)]
pub struct ButtonProps {
    #[props(default)]
    r#type: String,
    #[props(default)]
    name: String,
    #[props(default = false)]
    disabled: bool,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    children: Element,

    // Styling
    #[props(default)]
    variant: ButtonVariant,
    #[props(default)]
    size: ButtonSize,
    #[props(default)]
    class: String,
}

impl Component for ButtonProps {
    fn view(self) -> Element {
        let class = ButtonClass::builder()
            .variant(self.variant)
            .size(self.size)
            .with_class(self.class);

        rsx!(
            button {
                r#type: "{self.r#type}",
                name: "{self.name}",
                disabled: self.disabled,
                onclick: move |e| self.onclick.call(e),
                class: "{class}",
                {self.children}
            }
        )
    }
}
