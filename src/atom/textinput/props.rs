use component_derive::Component;

use crate::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct TextInputProps {
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(default = 0)]
    minlength: u32,
    #[props(default = 100)]
    maxlength: u32,
    #[props(default = "Placeholder...".to_string())]
    placeholder: String,
}

impl Component for TextInputProps {
    fn view(self) -> Element {
        let class = "textinput";
        rsx!(input {
            class: "{class}",
            r#type: "text",
            minlength: "{self.minlength}",
            maxlength: "{self.maxlength}",
            placeholder: "{self.placeholder}",
            oninput: move |event| self.oninput.call(event)
        })
    }
}
