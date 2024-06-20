use dioxus::prelude::*;
use dioxus_components::{atoms::Button, attributes::Color, molecules::{toast::use_toast, Toast}};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ToastPage() -> Element {
    let toast = use_toast();
    
    rsx!(
        Button {
            onclick: move |_| {
                toast(
                    Toast::default()
                        .title("Title")
                        .description(rsx! {
                            div { "Content" }
                        })
                        .color(Color::Default),
                )
            },
            "PopToast"
        }
    )
} 