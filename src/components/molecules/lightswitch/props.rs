use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

pub struct LightSwitchState {
    active: bool,
}

impl LightSwitchState {
    pub fn new(active: bool) -> Self {
        Self { active }
    }

    pub fn is_on(&self) -> Option<String> {
        if self.active {
            Some("dark".to_string())
        } else {
            None
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}

// REVIEW : Not finished, need to see in a real project how to properly handle states and such

#[props_component(class, id)]
pub fn LightSwitch() -> Element {
    let mut class = tw_merge!(props.class);

    let mut state = try_consume_context::<Signal<LightSwitchState>>();

    let onclick = move |_| {
        if let Some(state) = state.as_mut() {
            state.write().toggle();
        }
    };

    let icon = if let Some(state) = state.as_ref() {
        use_correct_theme_icon(*state)
    } else {
        class = tw_join!(class, "LightSwitch did not find its state");
        use_error_icon()
    };

    rsx!(
        button { r#type: "button", class, onclick: onclick, {icon} }
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

// <path xmlns="http://www.w3.org/2000/svg" d="M16.2426 7.75736C17.6695 9.18422 18.2275 11.1509 17.9166 13M7.75736 16.2426C5.41421 13.8995 5.41421 10.1005 7.75736 7.75732M4.92893 19.0711C1.02369 15.1658 1.02369 8.83418 4.92893 4.92893M19.0711 4.92898C21.9628 7.8207 22.7133 12.0428 21.3225 15.6251M10.5 10.6771C10.1888 11.0297 10 11.4928 10 12C10 13.1046 10.8954 14 12 14C12.5072 14 12.9703 13.8112 13.3229 13.5M21 21L3 3" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
fn use_error_icon() -> Element {
    rsx!(
        svg {
            view_box: "0 0 24 24",
            width: 24,
            height: 24,
            fill: "none",
            path {
                d: "M16.2426 7.75736C17.6695 9.18422 18.2275 11.1509 17.9166 13M7.75736 16.2426C5.41421 13.8995 5.41421 10.1005 7.75736 7.75732M4.92893 19.0711C1.02369 15.1658 1.02369 8.83418 4.92893 4.92893M19.0711 4.92898C21.9628 7.8207 22.7133 12.0428 21.3225 15.6251M10.5 10.6771C10.1888 11.0297 10 11.4928 10 12C10 13.1046 10.8954 14 12 14C12.5072 14 12.9703 13.8112 13.3229 13.5M21 21L3 3",
                stroke: "#000000",
                stroke_width: 2,
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
        }
    )
}

fn use_correct_theme_icon(state: Signal<LightSwitchState>) -> Element {
    rsx!(
        if !state.read().is_on().is_some() {
            svg {
                view_box: "0 0 24 24",
                width: 24,
                height: 24,
                stroke: "currentColor",
                stroke_width: 2,
                fill: "none",
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
                stroke: "currentColor",
                stroke_width: 2,
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
            }
        }
    )
}
