use dioxus::prelude::*;
use dioxus_components_bin::{components::form::checkbox::*, types::Color};

pub fn CheckboxPage() -> Element {
    rsx!(
        "CHECKBOX PAGE"
        div { class: "flex flex-col",
            div {
                label { r#for: "checkbox-1",
                    label { r#for: "checkbox-1", "Primary" }
                    Checkbox { id: "checkbox-1", checked: true, color: Color::Primary }
                }
            }
            div {
                Checkbox { id: "checkbox-2", checked: true, color: Color::Secondary }
                label { r#for: "checkbox-2", "Secondary" }
            }
            div {
                Checkbox { id: "checkbox-3", checked: true, color: Color::Destructive }
                label { class: "", r#for: "checkbox-3", "Destructive" }
            }
            div {
                Checkbox { id: "checkbox-4", checked: true, color: Color::Success }
                label { r#for: "checkbox-4", "Success" }
            }
            div {
                Checkbox { id: "checkbox-5", checked: true, disabled: true, color: Color::Primary }
                label { class: "peer-disabled:opacity-50", r#for: "checkbox-5", "Disabled" }
            }
        }
    )
}
