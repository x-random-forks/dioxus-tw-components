use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct TextInputProps {
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(default)]
    name: String,
    #[props(default = 0)]
    minlength: u32,
    #[props(default = 100)]
    maxlength: u32,
    #[props(default = "Placeholder...".to_string())]
    placeholder: String,
}

impl Component for TextInputProps {
    fn view(self) -> Element {
        let class = class![BaseClass::<TextInputProps>::Default];
        rsx!(input {
            r#type: "text",
            name: "{self.name}",
            minlength: "{self.minlength}",
            maxlength: "{self.maxlength}",
            placeholder: "{self.placeholder}",
            class: "{class}",
            oninput: move |event| self.oninput.call(event)
        })
    }
}
