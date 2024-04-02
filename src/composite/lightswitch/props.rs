use self::styling::Size;
use crate::{atom::icon::Icon, *};
use atom::icon::style::IconSvg;
use component_derive::Component;

pub use Size::{Lg, Md, Sm, Xl, Xs};

pub struct LightSwitchSignal(pub String);

// TODO Doc
#[derive(PartialEq, Props, Clone, Component)]
pub struct LightSwitchProps {
    // Styling
    #[props(default)]
    class: String,
}

impl Component for LightSwitchProps {
    fn view(self) -> Element {
        let light_switch_context = use_context::<Signal<LightSwitchSignal>>();

        let lightswitch_closure = move |_| {
            use_light_switch(light_switch_context);
        };

        let icon = get_day_icon(light_switch_context);

        let class = class!(self.class);

        rsx!(
            button {
                r#type: "button",
                class: "{class}",
                onclick: lightswitch_closure,
                Icon { svg: icon }
            }
        )
    }
}

// Switch the value of light switch to "dark" or ""
pub fn use_light_switch(mut light_switch_context: Signal<LightSwitchSignal>) {
    if light_switch_context.read().0.is_empty() {
        light_switch_context.write().0 = "dark".to_string();
    } else {
        light_switch_context.write().0 = "".to_string();
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
}

// Fetch in Local Storage last saved value of light_switch
pub fn use_user_pref_light() {
    let mut light_switch_context = use_context::<Signal<LightSwitchSignal>>();

    let _ = use_resource(move || async move {
        let mut eval = eval(
            r#"
            const light_switch_value = localStorage.getItem("light_switch");
            console.log(light_switch_value);
            dioxus.send(light_switch_value);
            "#,
        );

        if eval.recv().await.unwrap() == "dark" {
            light_switch_context.write().0 = "dark".to_string();
        } else {
            light_switch_context.write().0 = "".to_string();
        }
    });
}

fn get_day_icon(light_switch_context: Signal<LightSwitchSignal>) -> IconSvg {
    if light_switch_context.read().0.is_empty() {
        IconSvg::Sun
    } else {
        IconSvg::Moon
    }
}
