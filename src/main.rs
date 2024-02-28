#![allow(non_snake_case)]
use serde::Deserialize;
use std::hash::BuildHasherDefault;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{
    html::{button, img},
    prelude::*,
};

pub static BASE_API_URL: &str =
    "https://api.giphy.com/v2/emoji?api_key=qVfg42oatCA5UzDaMddHV2fFF5stwHaw&limit=10&offset=0";

#[derive(Deserialize, Clone)]
struct Data {
    data: Vec<Giphy>,
}

#[derive(Deserialize, Clone)]
struct Giphy {
    title: String,
}

fn main() {
    // launch the web app
    dioxus_logger::init(log::LevelFilter::Trace).expect("failed to init logger");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );
    launch(App);
}

// fn anon(e: Event<MouseData>) {}

// create a component that renders a div with the text "Hello, world!"
fn App() -> Element {
    rsx! {
        div {
            p {
                class: "text-zinc-600",
                "this is a test" }
            Gif{}
        }
    }
}

pub async fn get_emoji() -> Result<Data, reqwest::Error> {
    reqwest::get(format!("asd")).await?.json().await
}

fn Gif() -> Element {
    let gif = use_resource(move || get_emoji());

    match &*gif.value().read() {
        Some(Ok(data)) => rsx! {
            div {
                for item in data.data.clone() {
                    div {
                        class: "text-black-600",
                        ""
                    }
                }
            }
        },
        Some(Err(err)) => rsx! {"An error occurred while fetching stories {err}"},
        None => rsx! {"Loading items"},
    }
}
