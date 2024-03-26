use component_derive::Component;

use crate::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct TextAreaProps {
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(default = "Type your message here...".to_string())]
    placeholder: String,
    // Styling
}

impl Component for TextAreaProps {
    fn view(self) -> Element {
        rsx!(
            textarea {
                placeholder: "{self.placeholder}",
                class: "textarea",
                oninput: move |event| self.oninput.call(event)
            }
        )
    }
}
