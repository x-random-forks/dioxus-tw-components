use dioxus::prelude::*;
use dioxus_components_bin::composite::navbar::*;

pub fn NavbarPage() -> Element {
    let left_part = rsx!(
        div { class: "flex space-x-2 items-center ml-6 border border-border",
            div { "NAVBAR" }
            div { "Left" }
        }
    );
    let right_part = rsx!(
        div { class: "flex   items-center justify-end space-x-2 mr-6 border border-border",
            "RIGHT PART"
        }
    );

    rsx!(
            "NAVBAR PAGE"
            div { class: "border border-black", Navbar { left_part: left_part, right_part: right_part } }
    )
}
