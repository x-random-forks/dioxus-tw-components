use dioxus::prelude::*;
use dioxus_components_bin::atom::code::*;

pub fn CodePage() -> Element {
    rsx!(
        div { class: "h-screen",
            "CODE PAGE WIP"
            code { "test" }
        }
    )
}
