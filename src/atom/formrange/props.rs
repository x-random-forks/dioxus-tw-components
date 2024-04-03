use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct FormRangeProps {
    // Name corresponds to the key with its corresponding value that will be sent in the request eg name:value
    #[props(default)]
    name: String,
    #[props(default = 50)]
    value: i32,
    #[props(default = 0)]
    min: i32,
    #[props(default = 100)]
    max: i32,
    #[props(default = 1)]
    step: i32,
    #[props(default = false)]
    disabled: bool,
    // TODO implemant <datalist>
    #[props(default = "".to_string())]
    list: String,
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    // Styling TODO
    #[props(default)]
    class: String,
}

impl Component for FormRangeProps {
    fn view(self) -> Element {
        let class = FormRangeClass::builder().with_class(self.class);
        rsx!(
            input {
                r#type: "range",
                name: "{self.name}",
                min: "{self.min}",
                max: "{self.max}",
                step: "{self.step}",
                value: "{self.value}",
                disabled: "{self.disabled}",
                list: "{self.list}",
                class: "{class}",
                oninput: move |e| self.oninput.call(e)
            }
        )
    }
}

// TODO use <datalist> HTML attribute
