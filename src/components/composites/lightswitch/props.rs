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
    let class = tw_merge!(props.class);

    let mut state = consume_context::<Signal<LightSwitchState>>();

    let onclick = move |_| {
        state.write().toggle();
    };

    let icon = use_correct_theme_icon(state);

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

fn use_correct_theme_icon(state: Signal<LightSwitchState>) -> Element {
    rsx!(
        if !state.read().is_on().is_some() {
            dioxus_free_icons::Icon {
                class: "stroke-2",
                width: 24,
                height: 24,
                icon: dioxus_free_icons::icons::fi_icons::FiSun
            }
        } else {
            dioxus_free_icons::Icon {
                class: "",
                width: 24,
                height: 24,
                icon: dioxus_free_icons::icons::fi_icons::FiMoon
            }
        }
    )
}
