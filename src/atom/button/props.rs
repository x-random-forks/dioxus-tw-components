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
    #[props(default = ButtonSize::Md)]
    size: ButtonSize,
    #[props(default = ButtonVariant::Primary)]
    variant: ButtonVariant,
    #[props(default)]
    class: String,
    // color: ButtonColor,
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
