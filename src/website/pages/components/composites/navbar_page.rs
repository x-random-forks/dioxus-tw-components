use dioxus::prelude::*;
use dioxus_components_bin::components::composites::navbar::*;

pub fn NavbarPage() -> Element {
    rsx!(
        "NAVBAR PAGE"
        div { class: "border border-black", Navbar {} }
    )
}
