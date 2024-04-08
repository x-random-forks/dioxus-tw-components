use dioxus::prelude::*;
use dioxus_components_bin::atom::toggle::*;

pub fn TogglePage() -> Element {
    rsx!(
        "TOGGLE PAGE"
        div { class: "flex gap-4",
            div { class: "",
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: ToggleColor::Primary,
                    "Primary"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: ToggleColor::Secondary,
                    "Secondary"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: ToggleColor::Destructive,
                    "Destructive"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: ToggleColor::Primary,
                    disabled: true,
                    "Disabled"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: ToggleColor::Primary,
                    size: ToggleSize::Sm,
                    "Sm"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: ToggleColor::Primary,
                    size: ToggleSize::Md,
                    "Md"
                }
                Toggle {
                    name: "toggle",
                    value: "toggle",
                    checked: true,
                    color: ToggleColor::Primary,
                    size: ToggleSize::Lg,
                    "Lg"
                }
            }
        }
    )
}
