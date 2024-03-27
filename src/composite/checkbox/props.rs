use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct CheckboxProps {
    // What will be sent in the request eg name:value where value is represented by the selected CheckboxItem
    name: String,
    value: String,
    #[props(default = false)]
    checked: bool,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    required: bool,
    children: Element,
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    // Styling
}

impl Component for CheckboxProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<CheckboxProps>::Default);
        rsx!(
            div {
                input {
                    name: "{self.name}",
                    value: "{self.value}",
                    r#type: "checkbox",
                    checked: "{self.checked}",
                    disabled: "{self.disabled}",
                    required: "{self.required}",
                    class: "{class}",
                    oninput: move |e| self.oninput.call(e)
                }
                label { r#for: "{self.value}", {self.children} }
            }
        )
    }
}
