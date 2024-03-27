use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct FormRangeProps {
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    // Name corresponds to the key with its corresponding value that will be sent in the request eg name:value
    name: String,
    #[props(default = 0)]
    min: i32,
    #[props(default = 100)]
    max: i32,
    // Step when changing the value of the Range
    #[props(default = 1)]
    step: i32,
    // Default value of Range
    #[props(default = 50)]
    value: i32,
    // Styling
}

impl Component for FormRangeProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<FormRangeProps>::Default);
        rsx!(input {
            r#type: "range",
            name: "{self.name}",
            min: "{self.min}",
            max: "{self.max}",
            step: "{self.step}",
            value: "{self.value}",
            class: "{class}",
            oninput: move |e| self.oninput.call(e),
        })
    }
}
// #[props(default = "range".to_string())]
// r#type: String,
