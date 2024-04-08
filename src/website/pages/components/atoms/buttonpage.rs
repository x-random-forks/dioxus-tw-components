use dioxus::prelude::*;
use dioxus_components_bin::atom::button::*;

pub fn ButtonPage() -> Element {
    rsx!(
        div {
            Button { variant: ButtonVariant::Primary, "Default" }
            Button { variant: ButtonVariant::Primary, disabled: true, "Default" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
        }
        div {
            Button { size: ButtonSize::Md, "Md/Default" }
            Button { size: ButtonSize::Xs, "Xs" }
            Button { size: ButtonSize::Sm, "Sm" }
            Button { size: ButtonSize::Lg, "Lg" }
            Button { size: ButtonSize::Xl, "Xl" }
        }
    )
}
