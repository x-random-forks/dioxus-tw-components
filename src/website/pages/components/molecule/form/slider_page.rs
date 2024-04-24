use dioxus::prelude::*;
use dioxus_components::components::molecules::form::slider::*;

pub fn SliderPage() -> Element {
    rsx!(
        "SLIDER PAGE"
        Slider {}
        Slider { disabled: true }
    )
}
