use dioxus::prelude::*;
use dioxus_components_bin::{atom::button::*, composite::modal::*};

pub fn ModalPage() -> Element {
    rsx!(
        "MODAL PAGE"
        Modal { 
            ModalTrigger { 
                Button { variant: ButtonVariant::Outline, "Open Modal" }
            }
            ModalBackground {}
            ModalContent { 
                div {
                    ModalCancel { 
                        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Xs, "X" }
                    }
                }
                div { class: "h4", "TITLE" }
                div { class: "paragraph", "LONG LONG LONG LONG LONG LONG LONG LONG CONTENT" }
            }
        }
    )
}
