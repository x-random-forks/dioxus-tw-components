use dioxus::prelude::*;
use dioxus_components_bin::{attributes::Color, components::molecules::form::input::*};

pub fn InputPage() -> Element {
    rsx!(
        div { class: "flex flex-col space-y-2 w-52 p-4",
            "INPUT PAGE"
            div {
                Input { r#type: "text" }
            }
            div {
                Input { color: Color::Primary, r#type: "text" }
            }
            div {
                Input { color: Color::Secondary, r#type: "text" }
            }
            div {
                Input { color: Color::Destructive, r#type: "text" }
            }
            div {
                Input { color: Color::Success, r#type: "text" }
            }
            div {
                Input { color: Color::Accent, r#type: "text" }
            }
            div {
                Input { color: Color::Muted, r#type: "text" }
            }
            div {
                Input { r#type: "date" }
            }
            div {
                Input { r#type: "file" }
            }
            div {
                Input { r#type: "email", placeholder: "email" }
            }
            div {
                Input { r#type: "number", value: "2" }
            }
            div {
                Input { r#type: "text", disabled: true }
            }
            div {
                Input { r#type: "text" }
            }
        }
    )
}
