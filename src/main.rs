#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components_bin::atom::button::*;
use dioxus_components_bin::atom::Label;
use dioxus_components_bin::atom::RadioGroup;
use dioxus_components_bin::atom::RadioItem;
use dioxus_components_bin::atom::TextArea;
use dioxus_components_bin::atom::TextInput;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::WARN)
            .build(),
    );
    launch(App);
}

fn App() -> Element {
    // This should be a global context in something like an AppState
    // this is just for demonstration purpose
    let mut dark = use_signal(|| "".to_string());
    let lightswitch_closure = move |_| {
        log::debug!("LightSwitch clicked");
        if dark() == "" {
            dark.set("dark".to_string());
        } else {
            dark.set("".to_string());
        }
    };

    rsx!(
        body { class: "{dark} bg-background",
            div {
                Button { onclick: lightswitch_closure, "LightSwitch" }
            }
            TestForm {}
        }
    )
}

// With the override class implemented, the user could just do something like
// Button { class: Class("btn-primary btn-xl"), "My Button"}
fn TestButton() -> Element {
    let keyboard_closure = move |event: FormEvent| log::debug!("{}", event.value());
    rsx!(
        div { class: "",
            Button { color: Unset, "Unset" }
            Button { color: Primary, "Primary" }
            Button { color: Secondary, "Secondary" }
            Button { color: Accent, "Accent" }
            Button { variant: Outline(Primary), "Outline" }
            Button { variant: Outline(Secondary), "Outline Secondary" }
            Button { variant: Outline(Accent), "Outline Accent" }
            Button { variant: Ghost(Primary), "Ghost" }
            Button { variant: Ghost(Secondary), "Ghost Secondary" }
            Button { variant: Ghost(Accent), "Ghost Accent" }
            Button { size: Xs, variant: Ghost(Primary), "Ghost Primary Xs" }
        }
        div { class: "",
            Button { size: Sm, "Sm" }
            Button { "Default" }
            Button { size: Lg, "Lg" }
            Button { size: Xl, "Xl" }
        }
        div { class: "", TextArea { oninput: keyboard_closure } }
        div { class: "", TextInput { oninput: keyboard_closure } }
    )
}

fn TestForm() -> Element {
    rsx!(
        div { class: "",
            "TestForm"
            div { class: "", TestRadio {} }
        }
    )
}

fn TestRadio() -> Element {
    rsx!(
        div {
            form {
                method: "POST",
                id: "option-form",
                onsubmit: move |event| {
                    log::debug!("Form values {:#?}", event.values());
                    log::debug!("Valid {:#?}", event.valid());
                },
                // oninput: move |event| {
                //     log::debug!("Form {:#?}", event);
                // },
                RadioGroup { name: "option",
                    Label { r#for: "option", "Choose an option" }
                    div { class: "",
                        RadioItem { value: "option-1", name: "option" }
                        Label { r#for: "option", "Option 1" }
                    }
                    div {
                        RadioItem { value: "option-2", name: "option", checked: true }
                        Label { r#for: "option", "Option 2" }
                    }
                    div {
                        RadioItem { value: "option-3", name: "option", disabled: true }
                        Label { r#for: "option", "Option 3" }
                    }
                }
                RadioGroup { name: "gender",
                    Label { r#for: "gender", "Birth gender" }
                    div {
                        RadioItem { value: "male", name: "gender", required: true }
                        Label { r#for: "gender", "Male" }
                    }
                    div {
                        RadioItem { value: "female", name: "gender" }
                        Label { r#for: "gender", "Female" }
                    }
                }
                Button { variant: Ghost(Primary), size: Sm, r#type: "submit", name: "option", "Submit" }
            }
        }
    )
}
