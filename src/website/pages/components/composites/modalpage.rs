use dioxus::prelude::*;
use dioxus_components_bin::composite::modal::*;

pub fn ModalPage() -> Element {
    rsx!(
        div { class: "h-screen",
            "MODAL PAGE"
            Modal {}
        }
    )
}
