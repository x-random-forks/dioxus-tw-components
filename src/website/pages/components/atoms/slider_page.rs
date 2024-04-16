use dioxus::prelude::*;
use dioxus_components_bin::atom::slider::*;

pub fn SliderPage() -> Element {
    rsx!(
        "SLIDER PAGE"
        Slider {}
    )
}
