use dioxus::prelude::*;
use dioxus_components_bin::{
    components::form::toggle::*,
    types::{Color, Size},
};

pub fn TogglePage() -> Element {
    rsx!(
        "TOGGLE PAGE"
        div { class: "flex gap-4",
            div {
                Toggle { id: "toggle-1", checked: true, color: Color::Primary }
                label { r#for: "toggle-1", "Primary" }
            }
            div {
                Toggle { id: "toggle-2", checked: true, color: Color::Secondary }
                label { r#for: "toggle-2", "Secondary" }
            }
            div {
                Toggle { id: "toggle-3", checked: true, color: Color::Destructive }
                label { r#for: "toggle-3", "Destructive" }
            }
            div {
                Toggle { id: "toggle-4", checked: true, color: Color::Primary, disabled: true }
                label { class: "peer-disabled:opacity-50", r#for: "toggle-4", "Disabled" }
            }
            div {
                Toggle { id: "toggle-5", checked: true, color: Color::Primary, size: Size::Sm }
                label { r#for: "toggle-5", "Sm" }
            }
            div {
                Toggle { id: "toggle-6", checked: true, color: Color::Primary, size: Size::Md }
                label { r#for: "toggle-6", "Md" }
            }
        }
    )
}
