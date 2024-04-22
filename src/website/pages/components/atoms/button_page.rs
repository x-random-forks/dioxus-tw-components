use dioxus::prelude::*;
use dioxus_components_bin::{attributes::*, components::atom::button::*};

pub fn ButtonPage() -> Element {
    let onclick = move |_| {
        log::debug!("Primary button clicked");
    };

    rsx!(
        div { class: "grid space-y-4 border-4 border-primary",
            div {
                Button { "Default" }
                Button { color: Color::Primary, onclick: onclick, "Primary" }
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
            div {
                Button { animation: Animation::None, "None" }
                Button { animation: Animation::Light, "Light" }
                Button { animation: Animation::Full, "Full" }
                Button {
                    animation: Animation::Custom(
                        "relative overflow-hidden before:absolute before:inset-0 before:-translate-x-full before:bg-gradient-to-r before:from-transparent before:via-red-500/40 before:animate-[shimmer_2s_infinite]",
                    ),
                    "Custom"
                }
            }
        }
    )
}
