use crate::*;
use component_derive::Component;

use crate::atom::button::Button;

pub struct LightSwitchSignal(pub String);

// TODO Doc
#[derive(PartialEq, Props, Clone, Component)]
pub struct LightSwitchProps {}

impl Component for LightSwitchProps {
    fn view(self) -> Element {
        let mut dark_mode_context = consume_context::<Signal<LightSwitchSignal>>();

        let lightswitch_closure = move |_| {
            log::debug!("LightSwitch clicked");
            if dark_mode_context.read().0.is_empty() {
                dark_mode_context.write().0 = "dark".to_string();
            } else {
                dark_mode_context.write().0 = "".to_string();
            }
        };

        rsx!(div {
            Button { onclick: lightswitch_closure, "LightSwitch" },
        })
    }
}
