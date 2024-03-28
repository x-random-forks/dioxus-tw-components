#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_components_bin::atom::button::*;
use dioxus_components_bin::atom::checkbox::*;
use dioxus_components_bin::atom::formrange::*;
use dioxus_components_bin::atom::input::*;
use dioxus_components_bin::atom::label::*;
use dioxus_components_bin::atom::textarea::*;
use dioxus_components_bin::atom::toggle::*;
use dioxus_components_bin::composite::radiogroup::*;
use dioxus_components_bin::composite::select::*;
use InputType::*;

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
                    Input { r#type: Text, name: "username", placeholder: "username" }
                }
                div {
                    Label { r#for: "email", "Your email" }
                    Input { r#type: Email, name: "email", placeholder: "email" }
                }
                div {
                    Label { r#for: "age", "Your age" }
                    Input { r#type: Number, name: "age", placeholder: "age", min: 18, max: 100 }
                }
                div {
                    Label { r#for: "date", "Your Piscine date" }
                    Input { r#type: Date, name: "date", placeholder: "Piscine date" }
                }
                div {
                    Label { r#for: "message", "Send us a message" }
                    TextArea { name: "message", placeholder: "Your message..." }
                }
                div {
                    Label { r#for: "animal", "Select an animal" }
                    SelectGroup { name: "animal",
                        SelectPlaceholder { "Select an animal" }
                        SelectLabel { label: "Domestic",
                            SelectItem { value: "dog", "Dog" }
                            SelectItem { value: "cat", "Cat" }
                            SelectItem { value: "hamster", "Hamster" }
                            SelectItem { value: "none", disabled: true, "None" }
                        }
                        SelectLabel { label: "Wild", disabled: true,
                            SelectItem { value: "lion", "Lion" }
                            SelectItem { value: "tiger", "Tiger" }
                            SelectItem { value: "bear", "Bear" }
                        }
                    }
                }
                div {
                    Label { r#for: "activities", "Select your fav activities" }
                    Checkbox { name: "activities", value: "reading", "Reading" }
                    Checkbox { name: "activities", value: "coding", checked: true, "Coding" }
                    Checkbox { name: "activities", value: "writing", "Writing" }
                    Checkbox { name: "activities", value: "swimming", "Swimming" }
                    Checkbox { name: "activities", value: "football", "Football" }
                    Checkbox { name: "activities", value: "none", disabled: true, "None" }
                }
                div {
                    Label { r#for: "rate", "Rate us" }
                    FormRange { name: "rate", min: 0, max: 100, step: 1 }
                }
                div {
                    Toggle { name: "newsletter", value: "newsletter", checked: true,
                        Label { r#for: "newsletter", "Subscribe to newsletter" }
                    }
                }
                div {
                    Toggle { name: "toggle1", value: "toggle1", checked: true, color: Secondary,
                        Label { "toggle1" }
                    }
                }
                div {
                    Toggle {
                        name: "toggle2",
                        value: "toggle2",
                        checked: true,
                        color: Accent,
                        size: Sm,
                        Label { "toggle2" }
                    }
                }
                div {
                    Toggle { name: "toggle3", value: "toggle3", disabled: true, size: Lg,
                        Label { "toggle3" }
                    }
                }
                div {
                    Toggle { name: "cookie", value: "cookie", checked: false,
                        Label { "Use our cookie" }
                    }
                }
                div {
                    Checkbox { name: "terms", value: "yes", required: false,
                        Label { "Accept terms and conditions" }
                    }
                }
                div {
                    Button { variant: Ghost(Primary), size: Sm, r#type: "submit", "Submit" }
                }
            }
        }
        div { "Values: {values:#?}" }
    )
}
