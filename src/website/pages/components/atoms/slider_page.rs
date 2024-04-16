use dioxus::prelude::*;
use dioxus_components_bin::components::form::slider::*;

pub fn SliderPage() -> Element {
    rsx!(
        "SLIDER PAGE"
        Slider {}
        Slider { disabled: true }
    )
}
