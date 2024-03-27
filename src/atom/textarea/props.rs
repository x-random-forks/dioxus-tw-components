use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct TextAreaProps {
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(default)]
    name: String,
    #[props(default = "Type your text here...".to_string())]
    placeholder: String,
    // Styling
}

impl Component for TextAreaProps {
    fn view(self) -> Element {
        let class = class![BaseClass::<TextAreaProps>::Default];
        rsx!(textarea {
            placeholder: "{self.placeholder}",
            name: "{self.name}",
            class: "{class}",
            oninput: move |event| self.oninput.call(event)
        })
    }
}
