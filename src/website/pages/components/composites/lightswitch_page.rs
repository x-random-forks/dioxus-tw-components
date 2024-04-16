use dioxus::prelude::*;
use dioxus_components_bin::components::composite::lightswitch::*;

pub fn LightSwitchPage() -> Element {
    rsx!(
        "LIGHT SWITCH PAGE"
        div { class: "", LightSwitch { class: "size-10" } }
    )
}
