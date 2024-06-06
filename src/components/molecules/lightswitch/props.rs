use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;
use web_sys::HtmlElement;

use crate::attributes::*;
use crate::hooks::{use_document, use_window};

pub struct LightSwitchState {
    active: bool,
    dom_body: Option<HtmlElement>,
}

impl LightSwitchState {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            dom_body: None,
        }
    }

    pub fn set_body(&mut self, body: Option<HtmlElement>) {
        self.dom_body = body;
    }

    pub fn set_dark_theme(&mut self) {
        if let Some(body) = &self.dom_body {
            let mut body_class = body.class_name();
            // In case there is something else in the body class adds space before
            let dark = " dark";
            match body_class.find(dark) {
                None => {
                    self.active = true;
                    body_class.push_str(dark);
                }
                Some(index) => {
                    self.active = false;
                    body_class.replace_range(index..index + dark.len(), "");
                }
            }
            body.set_class_name(&body_class);
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

/// This component inserts/remove "dark" in the DOM body tag class
/// Uses only web_sys for now so it won't work for anything else than web
#[props_component(class, id, children)]
pub fn LightSwitch() -> Element {
    let mut state = use_signal(|| LightSwitchState::new(false));

    let onclick = move |_| {
        state.write().set_dark_theme();
    };

    let icon = use_correct_theme_icon(state);

    let onmounted = move |_| {
        let _ = use_window()
            .and_then(|window| use_document(&window))
            .map(|document| state.write().set_body(document.body()));
    };

    rsx!(
        button {
            r#type: "button",
            class: props.class,
            onclick,
            onmounted,
            {icon}
        }
    )
}

// let dark = light_switch_context.read().0.clone();
// let _ = use_resource(move || async move {
//     let eval = eval(
//         r#"
//         const html = document.querySelector("html");
//         const light_switch = "light_switch";
//         let color = await dioxus.recv();
//         if (color == "light") {
//             html.classList.remove("dark");
//             localStorage.setItem(light_switch, color);
//         } else {
//             html.classList.add("dark");
//             localStorage.setItem(light_switch, color);
//         }
//         "#,
//     );
//     let test = dark.read().0.clone().into();
//     log::debug!("JS Dark mode: {}", dark.read().0);
//     eval.send(test).unwrap();
// });

// Fetch in Local Storage last saved value of light_switch
// pub fn use_user_pref_light() {
//     let mut light_switch_context = use_context::<Signal<LightSwitchSignal>>();

//     let _ = use_resource(move || async move {
//         let mut eval = eval(
//             r#"
//             const light_switch_value = localStorage.getItem("light_switch");
//             console.log(light_switch_value);
//             dioxus.send(light_switch_value);
//             "#,
//         );

//         if eval.recv().await.unwrap() == "dark" {
//             light_switch_context.write().0 = "dark".to_string();
//         } else {
//             light_switch_context.write().0 = "".to_string();
//         }
//     });
// }

fn use_correct_theme_icon(state: Signal<LightSwitchState>) -> Element {
    rsx!(
        if !state.read().is_active() {
            svg {
                view_box: "0 0 24 24",
                width: 24,
                height: 24,
                // stroke: "currentColor",
                stroke_width: 2,
                fill: "none",
                class: "stroke-foreground",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: 12, cy: 12, r: 5 }
                line {
                    x1: 12,
                    y1: 1,
                    x2: 12,
                    y2: 3
                }
                line {
                    x1: 12,
                    y1: 21,
                    x2: 12,
                    y2: 23
                }
                line {
                    x1: 4.22,
                    y1: 4.22,
                    x2: 5.64,
                    y2: 5.64
                }
                line {
                    x1: 18.36,
                    y1: 18.36,
                    x2: 19.78,
                    y2: 19.78
                }
                line {
                    x1: 1,
                    y1: 12,
                    x2: 3,
                    y2: 12
                }
                line {
                    x1: 21,
                    y1: 12,
                    x2: 23,
                    y2: 12
                }
                line {
                    x1: 4.22,
                    y1: 19.78,
                    x2: 5.64,
                    y2: 18.36
                }
                line {
                    x1: 18.36,
                    y1: 5.64,
                    x2: 19.78,
                    y2: 4.22
                }
            }
        } else {
            svg {
                view_box: "0 0 24 24",
                width: 24,
                height: 24,
                class: "stroke-foreground",
                stroke_width: 2,
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
            }
        }
    )
}
