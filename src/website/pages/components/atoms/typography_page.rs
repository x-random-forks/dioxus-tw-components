use dioxus::prelude::*;

pub fn TypographyPage() -> Element {
    rsx!(
        div { class: "flex flex-col space-x-2",
            "TYPOGRAPHY PAGE"
            div { h1 { class: "h1", "h1" } }
            div { h2 { class: "h2", "h2" } }
            div { h3 { class: "h3", "h3" } }
            div { h4 { class: "h4", "h4" } }
            div { h5 { class: "h5", "h5" } }
            div { h6 { class: "h6", "h6" } }
            div { p { class: "paragraph", "paragraph" } }
            div { span { class: "span", "span" } }
            div { a { class: "anchor", "anchor" } }

            h3 { class: "h3", "Responsive text size" }
            div { p { class: "text-extrasmall", "text-extrasmall" } }
            div { p { class: "text-small", "text-small" } }
            div { p { class: "text-medium", "text-medium" } }
            div { p { class: "text-large", "text-large" } }
            div { p { class: "text-extralarge", "text-extralarge" } }
        }
    )
}
