use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use serde_json::Value;
use tailwind_fuse::*;

use crate::attributes::*;

pub struct LightSwitchState {
    active: bool,
}

impl LightSwitchState {
    pub fn new(active: bool) -> Self {
        Self { active }
    }

    fn toggle(&mut self) -> bool {
        self.active = !self.active;
        self.active
    }

    fn get_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

/// This component inserts/remove "dark" in the DOM on the div with id of main
#[props_component(class, id, children)]
pub fn LightSwitch() -> Element {
    let storage_dark_theme = use_resource(move || async move {
        // Get dark_theme from localStorage, if not found add it to false
        let mut eval = eval(
            r#"
            var dark_theme = localStorage.getItem("dark_theme");
            if (dark_theme == null) {
                var dark_theme = false;
                localStorage.setItem("dark_theme", dark_theme);
            } else {
                let main_div = document.getElementById("main");
                if (main_div != null && dark_theme == "true") {
                    main_div.classList.add("dark");
                }
            }
            dioxus.send(dark_theme);
            "#,
        );

        let dark = eval.recv().await;
        dark
    });

    let mut state = use_signal(|| LightSwitchState::new(false));

    use_effect(move || {
        match &*storage_dark_theme.read_unchecked() {
            Some(Ok(value)) => match value {
                Value::String(str) => {
                    let parsed_str = str.parse::<bool>();
                    if let Ok(bool_value) = parsed_str {
                        state.write().set_active(bool_value);
                    }
                }
                _ => {}
            },
            Some(Err(_)) => {}
            None => {}
        };
    });

    let onclick = move |_| {
        let dark_theme = state.write().toggle();
        spawn(async move {
            // Change value of dark_theme in localStorage
            let eval = eval(
                r#"
                const dark_theme = await dioxus.recv();
                localStorage.setItem("dark_theme", dark_theme);
                let main_div = document.getElementById("main");
                if (main != null) {
                    if (dark_theme) {
                        main_div.classList.add("dark");
                    } else {
                        main_div.classList.remove("dark");
                    }
                }
                "#,
            );
            let _ = eval.send(dark_theme.into());
        });
    };

    let icon = use_correct_theme_icon(state);

    rsx!(
        button {
            r#type: "button",
            class: props.class,
            onclick,
            {icon}
        }
    )
}

impl Named for LightSwitchProps {
    const NAME: &'static str = "LightSwitch";
}

fn use_correct_theme_icon(state: Signal<LightSwitchState>) -> Element {
    rsx!(
        if !state.read().get_active() {
            svg {
                view_box: "0 0 24 24",
                width: 24,
                height: 24,
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
