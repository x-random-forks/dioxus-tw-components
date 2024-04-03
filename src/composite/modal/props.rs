use super::style::*;
use crate::{atom::button::*, Component};
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq, Component)]
pub struct ModalProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for ModalProps {
    fn view(self) -> Element {
        // let mut sig = use_signal(|| false);

        // let button_closure = move |_| {
        //     sig.set(!sig());
        // };

        // rsx!(
        //     div {
        //         Button { class: "", onclick: button_closure, variant: ButtonVariant::Outline, "Modal" }
        //     }
        //     dialog { open: true,
        //         "DIALOG"
        //         form { method: "dialog", button { autofocus: true, "close" } }
        //     }
        //     if sig() {
        //         div { class: "fixed inset-0 bg-black bg-opacity-50",
        //             div { class: "fixed inset-0 flex justify-center items-center",
        //                 div { class: "bg-white p-4",
        //                     Button { onclick: button_closure, variant: ButtonVariant::Outline, "Close" }
        //                 }
        //             }
        //         }
        //     }
        // )
        rsx!(
            a { class:"anchor", href:"#dialog", "Open dialog" }
            dialog { id:"dialog", "DIALOG" a{href:"#!", "close"}}

        )
    }
}
