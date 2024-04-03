use dioxus::prelude::*;
use dioxus_components_bin::atom::input::*;

pub fn InputPage() -> Element {
    rsx!(
        div { class: "h-screen flex flex-col gap-2",
            "INPUT PAGE"
            div { Input { r#type: "text" } }
            div { Input { r#type: "date" } }
            div { Input { r#type: "email", placeholder: "email" } }
            div { Input { r#type: "number", value: "2" } }
            div { Input { r#type: "text", disabled: true } }
        }
    )
}
