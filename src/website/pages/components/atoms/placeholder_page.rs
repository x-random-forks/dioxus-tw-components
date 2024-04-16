use dioxus::prelude::*;
use dioxus_components_bin::atom::placeholder::*;

pub fn PlaceholderPage() -> Element {
    rsx!(
        "PLACEHOLDER PAGE"
        Placeholder { class: "rounded-global-radius w-20 h-20", animation: PlaceholderAnimation::None }
        Placeholder { class: "rounded-full w-40 h-40", animation: PlaceholderAnimation::Light }
        Placeholder { class: "rounded-global-radius w-80 h-4", animation: PlaceholderAnimation::Full }
        "No animation"
        Placeholder { class: "rounded-global-radius w-20 h-20" }
    )
}
