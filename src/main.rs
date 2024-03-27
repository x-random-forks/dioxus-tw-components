#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_components_bin::atom::button::*;
use dioxus_components_bin::atom::label::*;
use dioxus_components_bin::atom::radiogroup::*;
use dioxus_components_bin::atom::textarea::*;
use dioxus_components_bin::atom::textinput::*;

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
            div { TestForm {} }
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
    let mut values = use_signal(HashMap::new);
    rsx!(
        div {
            form {
                class: "border border-black w-96",
                method: "POST",
                id: "option-form",
                onsubmit: move |event| {
                    log::debug!("Form values {:#?}", values());
                    log::debug!("Valid :{}", event.valid());
                    let test = &event.values()["username"].as_value();
                    log::debug!("{:?}", test);
                    values.set(event.values());
                },
                // oninput: move |event| {
                //     values.set(event.values());
                // },
                RadioGroup { name: "gender", default_value: "male",
                    Label { r#for: "gender", "Choose birth gender" }
                    RadioItem { value: "male", name: "gender", required: true, "Male" }
                    RadioItem { value: "female", name: "gender", "Female" }
                    RadioItem { value: "other", name: "gender", "Other" }
                    RadioItem { value: "disabled", name: "gender", disabled: true, "Disabled" }
                }
                div { class: "flex flex-col",
                    Label { r#for: "username", "Your username" }
                    TextInput { name: "username", placeholder: "username" }
                }
                div {
                    Label { r#for: "message", "Send us a message" }
                    TextArea { name: "message", placeholder: "Your message..." }
                }
                div {
                    Button {  variant: Ghost(Primary), size: Sm, r#type: "submit", "Submit" }
                }
            }
        }
        div { "Values: {values:#?}" }
    )
}
