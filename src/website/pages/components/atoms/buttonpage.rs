use dioxus::prelude::*;
use dioxus_components_bin::atom::button::*;

pub fn ButtonPage() -> Element {
    rsx!(
        div { class: "grid space-y-4 border-4 border-primary",
            div {
                Button { "Default" }
                Button { color: ButtonColor::Primary, "Primary" }
                Button { color: ButtonColor::Secondary, "Secondary" }
                Button { color: ButtonColor::Destructive, "Destructive" }
            }
            div { class: "",
                Button { variant: ButtonVariant::Outline, "Outline" }
                Button { variant: ButtonVariant::Outline, color: ButtonColor::Primary, "Primary" }
                Button { variant: ButtonVariant::Outline, color: ButtonColor::Secondary, "Secondary" }
                Button { variant: ButtonVariant::Outline, color: ButtonColor::Destructive, "Destructive" }
            }
            div {
                Button { variant: ButtonVariant::Ghost, "Ghost" }
                Button { variant: ButtonVariant::Ghost, color: ButtonColor::Primary, "Primary" }
                Button { variant: ButtonVariant::Ghost, color: ButtonColor::Secondary, "Secondary" }
                Button { variant: ButtonVariant::Ghost, color: ButtonColor::Destructive, "Destructive" }
            }
            div {
                Button { disabled: true, "Default" }
                Button { disabled: true, color: ButtonColor::Primary, "Primary" }
                Button { disabled: true, color: ButtonColor::Secondary, "Secondary" }
                Button { disabled: true, color: ButtonColor::Destructive, "Destructive" }
            }
            div {
                Button { size: ButtonSize::Md, "Md/Default" }
                Button { size: ButtonSize::Xs, "Xs" }
                Button { size: ButtonSize::Sm, "Sm" }
                Button { size: ButtonSize::Lg, "Lg" }
                Button { size: ButtonSize::Xl, "Xl" }
            }
        }
    )
}
