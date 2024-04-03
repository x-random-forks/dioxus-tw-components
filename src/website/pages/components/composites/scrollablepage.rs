use dioxus::prelude::*;
use dioxus_components_bin::composite::scrollable::*;

pub fn ScrollablePage() -> Element {
    rsx!(
        div { class: "h-screen flex flex-col gap-2",
            "SCROLLABLE PAGE"
            Scrollable { class: "h-64 w-64 bg-gray-200",
                div { class: "h-64 w-full bg-gray-300 border border-black", "SCROLLABLE CONTENT" }
                div { class: "h-64 w-full bg-gray-300 border border-black", "SCROLLABLE CONTENT" }
                div { class: "h-64 w-full bg-gray-300 border border-black", "SCROLLABLE CONTENT" }
                div { class: "h-64 w-full bg-gray-300 border border-black", "SCROLLABLE CONTENT" }
            }
        }
    )
}
