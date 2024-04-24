use dioxus::prelude::*;
use dioxus_components::components::atoms::label::Label;
use dioxus_components::components::molecules::form::input::Input;
use dioxus_components::layout::*;

pub fn TmpPage() -> Element {
    rsx!(
        div {
            "TMP PAGE"
            div { class: "grid gap-2",
                Tmp { layout: TmpEnum::Inline,
                    Label { "Enter your name" }
                    Input { placeholder: "Name" }
                }
                Tmp { layout: TmpEnum::Vertical,
                    Label { "Enter your name" }
                    Input { placeholder: "Name" }
                }
            }
        }
    )
}
