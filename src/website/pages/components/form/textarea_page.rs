use dioxus::prelude::*;
use dioxus_components_bin::{components::form::textarea::*, types::Color};

pub fn TextAreaPage() -> Element {
    rsx!(
        "TEXTAREA PAGE"
        div { class: "grid space-y-2",
            TextArea { color: Color::Default, placeholder: "Abc" }
            TextArea { color: Color::Primary }
            TextArea { color: Color::Secondary }
            TextArea { color: Color::Destructive }
            TextArea { color: Color::Success }
            TextArea { disabled: true }
        }
    )
}
