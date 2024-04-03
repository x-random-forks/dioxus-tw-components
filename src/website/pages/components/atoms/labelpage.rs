use dioxus::prelude::*;
use dioxus_components_bin::atom::label::*;

pub fn LabelPage() -> Element {
    rsx!(
        div { class: "h-screen ",
            "LABEL PAGE"
            div { class: "",
                Label { "Default" }
                Label { color: LabelColor::Primary, "Primary" }
                Label { color: LabelColor::Secondary, "Secondary" }
                Label { color: LabelColor::Accent, "Accent" }
            }
        }
    )
}
