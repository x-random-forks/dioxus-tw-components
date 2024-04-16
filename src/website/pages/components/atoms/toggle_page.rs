use dioxus::prelude::*;
use dioxus_components_bin::{
    components::form::toggle::*,
    types::{Color, Size},
};

pub fn TogglePage() -> Element {
    rsx!(
        "TOGGLE PAGE"
        div { class: "flex gap-4",
            div { class: "",
                Toggle { name: "toggle", value: "toggle", checked: true, color: Color::Primary, "Primary" }
                Toggle { name: "toggle", value: "toggle", checked: true, color: Color::Secondary, "Secondary" }
                Toggle { name: "toggle", value: "toggle", checked: true, color: Color::Destructive, "Destructive" }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: Color::Primary,
                    disabled: true,
                    "Disabled"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: Color::Primary,
                    size: Size::Sm,
                    "Sm"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: Color::Primary,
                    size: Size::Md,
                    "Md"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: Color::Primary,
                    size: Size::Lg,
                    "Lg"
                }
            }
        }
    )
}
