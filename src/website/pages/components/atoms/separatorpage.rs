use dioxus::prelude::*;
use dioxus_components_bin::atom::separator::*;

pub fn SeparatorPage() -> Element {
    rsx!(
        div { class: "h-screen ",
            "SEPARATOR PAGE"
            div {
                div { class: "inline-block",
                    p { "Dioxus comp lib" }
                    Separator { class: "my-4" }
                    div { class: "flex h-5 items-center space-x-4 text-sm",
                        div { "AAAAA" }
                        Separator { vertical: true }
                        div { "BBBBBBBBBBBBBBBBBBBBBBB" }
                        Separator { vertical: true }
                        div { "ZZ" }
                    }
                }
            }
        }
    )
}
