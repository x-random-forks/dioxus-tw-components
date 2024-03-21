use component_derive::Component;

use crate::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct ButtonProps {
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    children: Element,
    // Styling
    #[props(default)]
    variant: ButtonVariant,
    #[props(default)]
    size: ButtonSize,
}

impl Component for ButtonProps {
    fn view(self) -> Element {
        let variant = self.variant.class_string();
        let size = self.size.class_string();

        let class = format!("{} {}", variant, size);
        rsx!(
            button {
                class: "{class}",
                onclick: move |e| { self.onclick.call(e) },
                {self.children}
            }
        )
    }
}

#[derive(PartialEq, Clone, Default)]
pub enum ButtonVariant {
    Default,
    #[default]
    Primary,
    Secondary,
    Outline,
}

// Variant Trait ?

impl ButtonVariant {
    fn class_string(&self) -> String {
        match self {
            ButtonVariant::Default => "btn-base",
            ButtonVariant::Primary => "btn-primary",
            ButtonVariant::Secondary => "btn-secondary",
            ButtonVariant::Outline => "btn-outline",
        }
        .to_string()
    }
}

#[derive(PartialEq, Clone, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ButtonSize {
    fn class_string(&self) -> String {
        match self {
            ButtonSize::Sm => "btn-sm",
            // Default
            ButtonSize::Md => "",
            ButtonSize::Lg => "btn-lg",
            ButtonSize::Xl => "btn-xl",
        }
        .to_string()
    }
}
