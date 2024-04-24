use dioxus::prelude::*;
use dioxus_components_bin::{
    attributes::{Color, Size},
    components::{atoms::label::*, molecules::form::toggle::*},
};

pub fn TogglePage() -> Element {
    rsx!(
        "TOGGLE PAGE"
        div { class: "flex gap-4",
            div {
                Toggle { id: "toggle-1", color: Color::Primary }
                Label { r#for: "toggle-1", "Primary" }
            }
            div {
                Toggle { id: "toggle-2", color: Color::Secondary }
                Label { r#for: "toggle-2", "Secondary" }
            }
            div {
                Toggle { id: "toggle-3", color: Color::Destructive }
                Label { r#for: "toggle-3", "Destructive" }
            }
            div {
                Toggle { id: "toggle-4", color: Color::Primary, disabled: true }
                Label { class: "peer-disabled:opacity-50", r#for: "toggle-4", "Disabled" }
            }
            div {
                Toggle { id: "toggle-5", size: Size::Sm }
                Label { r#for: "toggle-5", "Sm" }
            }
            div {
                Toggle { id: "toggle-6", size: Size::Md }
                Label { r#for: "toggle-6", "Md" }
            }
            div {
                Toggle { id: "toggle-7", size: Size::Lg }
                Label { r#for: "toggle-7", "Lg" }
            }
        }
    )
}
