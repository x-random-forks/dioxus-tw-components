use dioxus::prelude::*;
use dioxus_components_bin::composite::lightswitch::*;

pub fn LightSwitchPage() -> Element {
    rsx!(
        div { class: "h-screen",
            "LIGHT SWITCH PAGE"
            div { class: "", LightSwitch {class: "size-10"} }
        }
    )
}
