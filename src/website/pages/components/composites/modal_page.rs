use dioxus::prelude::*;
use dioxus_components_bin::components::composites::modal::*;
use dioxus_components_bin::{attributes::Size, components::atoms::button::*};

pub fn ModalPage() -> Element {
    rsx!(
        "MODAL PAGE"
        Modal { 
            ModalTrigger { 
                Button { variant: ButtonVariant::Outline, "Open Modal" }
            }
            ModalBackground {}
            ModalContent { 
                div { ModalClose { 
                    Button { variant: ButtonVariant::Ghost, size: Size::Xs, "X" }
                } }
                div { class: "h4", "TITLE" }
                div { class: "paragraph", "LONG LONG LONG LONG LONG LONG LONG LONG CONTENT" }
            }
        }
    )
}
