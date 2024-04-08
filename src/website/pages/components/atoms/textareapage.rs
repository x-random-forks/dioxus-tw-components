use dioxus::prelude::*;
use dioxus_components_bin::atom::textarea::*;

pub fn TextAreaPage() -> Element {
    rsx!(
            "TEXTAREA PAGE"
            TextArea {}
            TextArea { disabled: true }
    )
}
