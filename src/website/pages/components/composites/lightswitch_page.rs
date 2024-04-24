use dioxus::prelude::*;
use dioxus_components::components::composites::lightswitch::*;

pub fn LightSwitchPage() -> Element {
    rsx!(
        "LIGHT SWITCH PAGE"
        div { class: "",
            LightSwitch { class: "size-10" }
        }
    )
}
