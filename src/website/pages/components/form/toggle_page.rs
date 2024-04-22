use dioxus::prelude::*;
use dioxus_components_bin::{
    attributes::{Color, Size},
    components::form::toggle::*,
};

pub fn TogglePage() -> Element {
    rsx!(
        "TOGGLE PAGE"
        div { class: "flex gap-4",
            div {
                Toggle { id: "toggle-1", color: Color::Primary }
                label { class: "select-none", r#for: "toggle-1", "Primary" }
            }
            div {
                Toggle { id: "toggle-2", color: Color::Secondary }
                label { r#for: "toggle-2", "Secondary" }
            }
            div {
                Toggle { id: "toggle-3", color: Color::Destructive }
                label { r#for: "toggle-3", "Destructive" }
            }
            div {
                Toggle { id: "toggle-4", color: Color::Primary, disabled: true }
                label { class: "peer-disabled:opacity-50", r#for: "toggle-4", "Disabled" }
            }
            div {
                Toggle { id: "toggle-5", size: Size::Sm }
                label { r#for: "toggle-5", "Sm" }
            }
            div {
                Toggle { id: "toggle-6", size: Size::Md }
                label { r#for: "toggle-6", "Md" }
            }
            div {
                Toggle { id: "toggle-7", size: Size::Lg }
                label { r#for: "toggle-7", "Lg" }
            }
        }
    )
}
