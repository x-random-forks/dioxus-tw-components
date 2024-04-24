use dioxus::prelude::*;
use dioxus_components_bin::{attributes::Orientation, components::atoms::separator::*};

pub fn SeparatorPage() -> Element {
    rsx!(
        "SEPARATOR PAGE"
        div {
            div { class: "inline-block",
                p { "Dioxus comp lib" }
                Separator { class: "my-4" }
                div { class: "flex h-5 items-center space-x-4 text-sm",
                    div { "AAAAA" }
                    Separator { orientation: Orientation::Vertical }
                    div { "BBBBBBBBBBBBBBBBBBBBBBB" }
                    Separator { orientation: Orientation::Vertical }
                    div { "ZZ" }
                }
            }
        }
    )
}
