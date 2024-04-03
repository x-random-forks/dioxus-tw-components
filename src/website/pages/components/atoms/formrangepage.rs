use dioxus::prelude::*;
use dioxus_components_bin::atom::formrange::*;

pub fn FormRangePage() -> Element {
    rsx!(
        div { class: "h-screen ",
            "FORMRANGE PAGE"
            FormRange {}
        }
    )
}
