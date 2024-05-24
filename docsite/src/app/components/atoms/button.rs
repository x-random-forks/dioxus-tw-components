use dioxus::prelude::*;
use dioxus_components::{atoms::Button, attributes::{Color, Size}, form::{SelectGroup, SelectItem}};

#[component]
pub fn ButtonPage() -> Element {
    rsx!(
        h1 { class: "h1", "Button" }
        p { class: "paragraph", "A simple yet customizable button" }
        p { class: "paragraph anchor", "Source should be outlink" }
        p { class: "paragraph anchor", "Page Source should outlink" }
        DemoTemplate { title: "Button" }
    )
}

#[component]
pub fn DemoTemplate(#[props(optional)] title: String, children: Element) -> Element {
    let mut color = use_signal(|| Color::default());
    let mut size = use_signal(|| Size::default());

    rsx!(
        section { class: "w-full border border-orange-700",
            h2 { class: "sr-only", { title } }
            div { class: "border border-yellow-500", "header" }
            div { class: "flex items-center justify-center", PreviewTemplate { 
                Button { color: color(), size: Size::Xl, "Button" }
            } }
            div { class: "border border-red-600", "footer" }
            SelectGroup { oninput: move |event: FormEvent| { color.set(Color::str_to_color(event.value())) },
                SelectItem { value: "default", "Default" }
                SelectItem { value: "primary", "Primary" }
                SelectItem { value: "secondary", "Secondary" }
                SelectItem { value: "success", "Success" }
                SelectItem { value: "destructive", "Destructive" }
            }
            SelectGroup { oninput: move |event: FormEvent| { size.set(Size::str_to_size(event.value())); log::debug!("size {:?}", event.value()) },
                SelectItem { value: "default", "Default / Medium" }
                SelectItem { value: "xs", "Extra-Small" }
                SelectItem { value: "sm", "Small" }
                SelectItem { value: "lg", "Large" }
                SelectItem { value: "xl", "Extra-Large" }
            }
        }
    )
}

#[component]
pub fn PreviewTemplate(children: Element) -> Element {
    rsx!(
        div { class: "p-4", { children } }
    )
}

