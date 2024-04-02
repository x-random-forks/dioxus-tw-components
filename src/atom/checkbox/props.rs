use self::styling::{BaseClass, Color};
use crate::*;
use component_derive::Component;

pub use Color::{Accent, Primary, Secondary, Unset};

#[derive(PartialEq, Props, Clone, Component)]
pub struct CheckboxProps {
    // Name of input field, associate with its value when sending the associated form
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
    #[props(default = Color::Primary)]
    color: Color<CheckboxProps>,
}

impl Component for CheckboxProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<CheckboxProps>::BaseClass, self.color);
        rsx!(
            label { class: "cursor-pointer gap-x-1 flex items-center",
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
                div { class: "peer-disabled:opacity-30", {self.children} }
            }
        )
    }
}
