use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct CheckboxProps {
    // Name of input field, associate with its value when sending the associated form
    #[props(default)]
    name: String,
    #[props(default)]
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
    #[props(default)]
    color: CheckboxColor,
    #[props(default)]
    class: String,
}

impl Component for CheckboxProps {
    fn view(self) -> Element {
        let class = CheckboxClass::builder()
            .color(self.color)
            .with_class(self.class);

        rsx!(
            label { class: "cursor-pointer gap-x-1 flex items-center",
                input {
                    name: "{self.name}",
                    value: "{self.value}",
                    r#type: "checkbox",
                    checked: self.checked,
                    disabled: self.disabled,
                    required: self.required,
                    class: "{class}",
                    oninput: move |e| self.oninput.call(e)
                }
                div { class: "peer-disabled:opacity-30", {self.children} }
            }
        )
    }
}
