use dioxus::prelude::*;
use dioxus_components_bin::{components::atom::placeholder::*, types::Animation};

pub fn PlaceholderPage() -> Element {
    rsx!(
        "PLACEHOLDER PAGE"
        Placeholder { class: "rounded-global-radius", width: 40, animation: Animation::None }
        Placeholder { class: "rounded-full w-40 h-40", animation: Animation::Light }
        Placeholder { class: "rounded-global-radius w-80 h-4", animation: Animation::Full }
        Placeholder {
            class: "rounded-global-radius w-80 h-4",
            animation: Animation::Custom(
                "relative overflow-hidden before:absolute before:inset-0 before:-translate-x-full before:bg-gradient-to-r before:from-transparent before:via-red-500/40 before:animate-[shimmer_2s_infinite]",
            )
        }
        "No animation"
        Placeholder { class: "rounded-global-radius w-20 h-20" }
    )
}
