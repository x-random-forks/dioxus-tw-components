use dioxus::prelude::*;
use dioxus_components::form::Input;

use crate::app::doctrait::{DemoComp, DemoState};

#[component]
pub fn PreviewFull<T: DemoComp + 'static>() -> Element {
    rsx!(
        h1 { class: "h1", {T::title()} }
        p { class: "paragraph", "A simple yet customizable button" }
        PreviewDemo::<T> {}
    )
}
#[component]
fn PreviewDemo<T: DemoComp>() -> Element {
    let demo_state = DemoState::new();

    rsx!(
        section { class: "w-full border border-orange-700",
            h2 { class: "sr-only", {T::title()} }
            div { class: "border border-yellow-500", "header" }
            div { class: "flex items-center justify-center", PreviewWindow { {T::preview_comp(&demo_state)} } }
            div { class: "border border-red-600", "footer" }
            {T::select_attributes(&demo_state)}
        }
    )
}

#[component]
pub fn PreviewGroupAttr(children: Element) -> Element {
    rsx!(
        div { class: "grid grid-cols-4 gap- grid-auto-rows auto-flow-dense w-full",
            { children }
        }
    )
}

#[component]
pub fn PreviewWindow(children: Element) -> Element {
    rsx!(
        div { class: "p-4 min-h-64 border border-black flex justify-center items-center",
            { children }
        }
    )
}

#[component]
pub fn PreviewClass(signal_class: Signal<String>, signal_override: Signal<String>) -> Element {
    rsx!(
        div { class: "flex flex-row items-center gap-2",
            "Class"
            Input { oninput: move |event: FormEvent| { signal_class.set(event.data().value()) } }
        }
        div { class: "flex flex-row items-center gap-2",
            "Override"
            Input { oninput: move |event: FormEvent| { signal_override.set(event.data().value()) } }
        }
    )
}
