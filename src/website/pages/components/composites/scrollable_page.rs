use dioxus::prelude::*;
use dioxus_components_bin::{attributes::Orientation, components::composites::scrollable::*};

pub fn ScrollablePage() -> Element {
    rsx!(
        div { class: "flex gap-2",
            "SCROLLABLE PAGE"
            Scrollable { class: "h-80 w-36 bg-gray-200",
                div { class: "h-64 w-full bg-gray-300 border border-black", "SCROLLABLE CONTENT" }
                div { class: "h-64 w-full bg-gray-300 border border-black", "SCROLLABLE CONTENT" }
                div { class: "h-64 w-full bg-gray-300 border border-black", "SCROLLABLE CONTENT" }
                div { class: "h-64 w-full bg-gray-300 border border-black", "SCROLLABLE CONTENT" }
            }
            // WIP
            "Horizontal WIP"
            Scrollable { class: "h-80 w-32 bg-gray-200", orientation: Orientation::Horizontal,
                for i in 0..10 {
                    div { class: "h-24 w-24 bg-gray-300 border border-black rotate-90 origin-[right_top]",
                        "SCROLLABLE {i} CONTENT"
                    }
                }
            }
        }
    )
}
