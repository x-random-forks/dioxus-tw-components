use dioxus::prelude::*;
use dioxus_components_bin::atom::input::*;

pub fn InputPage() -> Element {
    rsx!(
        div { class: "flex flex-col space-y-2",
            "INPUT PAGE"
            div { Input { r#type: "text" } }
            div { Input { r#type: "date" } }
            div { Input { r#type: "file" } }
            div { Input { r#type: "email", placeholder: "email" } }
            div { Input { r#type: "number", value: "2" } }
            div { Input { r#type: "text", disabled: true } }
        }
    )
}
