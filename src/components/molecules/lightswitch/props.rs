use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use serde_json::Value;

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

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct LightSwitchProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    pub onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for LightSwitchProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: Ok(VNode::default()),
        }
    }
}

/// This component inserts/remove "dark" in the DOM on the div with id of main
pub fn LightSwitch(mut props: LightSwitchProps) -> Element {
    props.update_class_attribute();

    let storage_dark_theme = use_resource(move || async move {
        // Get dark_theme from localStorage, if not found add it to false
        let mut eval = document::eval(
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

        eval.recv().await
    });

    let mut state = use_signal(|| LightSwitchState::new(false));

    use_effect(move || {
        if let Some(Ok(Value::String(str))) = &*storage_dark_theme.read_unchecked() {
            let parsed_str = str.parse::<bool>();
            if let Ok(bool_value) = parsed_str {
                state.write().set_active(bool_value);
            }
        };
    });

    let mut onclick = move |_| {
        let dark_theme = state.write().toggle();
        spawn(async move {
            // Change value of dark_theme in localStorage
            let eval = document::eval(
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
            let _ = eval.send(dark_theme);
        });
    };

    let icon = svg_icon(state);

    rsx!(
        button {
            r#type: "button",
            onclick: move |e| {
                if props.onclick != EventHandler::<MouseEvent>::default() {
                    state.write().toggle();
                    props.onclick.call(e);
                } else {
                    onclick(e)
                }
            },
            ..props.attributes,
            {icon}
        }
    )
}

fn svg_icon(state: Signal<LightSwitchState>) -> Element {
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
                    y2: 3,
                }
                line {
                    x1: 12,
                    y1: 21,
                    x2: 12,
                    y2: 23,
                }
                line {
                    x1: 4.22,
                    y1: 4.22,
                    x2: 5.64,
                    y2: 5.64,
                }
                line {
                    x1: 18.36,
                    y1: 18.36,
                    x2: 19.78,
                    y2: 19.78,
                }
                line {
                    x1: 1,
                    y1: 12,
                    x2: 3,
                    y2: 12,
                }
                line {
                    x1: 21,
                    y1: 12,
                    x2: 23,
                    y2: 12,
                }
                line {
                    x1: 4.22,
                    y1: 19.78,
                    x2: 5.64,
                    y2: 18.36,
                }
                line {
                    x1: 18.36,
                    y1: 5.64,
                    x2: 19.78,
                    y2: 4.22,
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
