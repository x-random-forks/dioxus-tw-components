use dioxus::prelude::*;
use dioxus_components_bin::{components::atom::placeholder::*, types::Animation};

pub fn PlaceholderPage() -> Element {
    rsx!(
        "PLACEHOLDER PAGE"
        Placeholder {
            class: "rounded-global-radius",
            width: 40,
            animation: Animation::None
        }
        Placeholder { class: "rounded-full w-40 h-40", animation: Animation::Light }
        Placeholder { class: "rounded-global-radius w-80 h-4", animation: Animation::Full }
        "No animation"
        Placeholder { class: "rounded-global-radius w-20 h-20" }
    )
}
