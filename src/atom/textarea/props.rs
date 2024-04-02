use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct TextAreaProps {
    #[props(default)]
    name: String,
    #[props(default)]
    value: String,
    #[props(default = "Type your text here...".to_string())]
    placeholder: String,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    required: bool,
    #[props(default = false)]
    readonly: bool,
    #[props(default = 0)]
    minlength: u32,
    #[props(default = 100000)]
    maxlength: u32,
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    // Styling
}

impl Component for TextAreaProps {
    fn view(self) -> Element {
        let class = class![BaseClass::<TextAreaProps>::BaseClass];
        rsx!(
            textarea {
                name: "{self.name}",
                value: "{self.value}",
                placeholder: "{self.placeholder}",
                disabled: "{self.disabled}",
                minlength: "{self.minlength}",
                maxlength: "{self.maxlength}",
                class: "{class}",
                oninput: move |event| self.oninput.call(event)
            }
        )
    }
}
