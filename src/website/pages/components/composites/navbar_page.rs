use dioxus::prelude::*;
use dioxus_components::components::composites::navbar::*;

pub fn NavbarPage() -> Element {
    rsx!(
        "NAVBAR PAGE"
        div { class: "border border-black", Navbar {} }
    )
}
