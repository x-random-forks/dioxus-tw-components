use dioxus::prelude::*;
use dioxus_components_bin::{atom::button::*, types::*};

pub fn ButtonPage() -> Element {
    rsx!(
        div { class: "grid space-y-4 border-4 border-primary",
            div {
                Button { "Default" }
                Button { color: Color::Primary, "Primary" }
                Button { color: Color::Secondary, "Secondary" }
                Button { color: Color::Destructive, "Destructive" }
            }
            div { class: "",
                Button { variant: ButtonVariant::Outline, "Outline" }
                Button { variant: ButtonVariant::Outline, color: Color::Primary, "Primary" }
                Button { variant: ButtonVariant::Outline, color: Color::Secondary, "Secondary" }
                Button { variant: ButtonVariant::Outline, color: Color::Destructive, "Destructive" }
            }
            div {
                Button { variant: ButtonVariant::Ghost, "Ghost" }
                Button { variant: ButtonVariant::Ghost, color: Color::Primary, "Primary" }
                Button { variant: ButtonVariant::Ghost, color: Color::Secondary, "Secondary" }
                Button { variant: ButtonVariant::Ghost, color: Color::Destructive, "Destructive" }
            }
            div {
                Button { disabled: true, "Default" }
                Button { disabled: true, color: Color::Primary, "Primary" }
                Button { disabled: true, color: Color::Secondary, "Secondary" }
                Button { disabled: true, color: Color::Destructive, "Destructive" }
            }
            div {
                Button { size: Size::Md, "Md/Default" }
                Button { size: Size::Xs, "Xs" }
                Button { size: Size::Sm, "Sm" }
                Button { size: Size::Lg, "Lg" }
                Button { size: Size::Xl, "Xl" }
            }
        }
    )
}
