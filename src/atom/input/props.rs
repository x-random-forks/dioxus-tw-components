use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct InputProps {
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(default)]
    r#type: String,
    // Name of input field, associate with its value when sending the associated form
    #[props(default)]
    name: String,
    // Initial value of the field
    #[props(default)]
    value: String,
    #[props(default)]
    id: String,
    // Min length of value(String) attribute (only for text, email)
    #[props(default = 0)]
    minlength: u32,
    // Max length of value(String) attribute (only for text, email)
    #[props(default = 100)]
    maxlength: u32,
    // Min value (only for number, date) can be negative
    #[props(default = 0)]
    min: i32,
    // Min value (only for number, date) can be negative
    #[props(default = 100)]
    max: i32,
    // Placeholder text when there is no value in input field (only for text, email, number)
    #[props(default = "Placeholder...".to_string())]
    placeholder: String,
    #[props(default = false)]
    required: bool,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    readonly: bool,
    // Styling

    // For Listing, this overrides the default base class
    // Temporary
    #[props(default)]
    groupclass: String,
}

impl Component for InputProps {
    fn view(self) -> Element {
        // TODO
        let class;
        if self.groupclass.is_empty() {
            class = class![BaseClass::<InputProps>::Default];
        } else {
            class = class![self.groupclass];
        }
        rsx! {
            input {
                r#type: "{self.r#type}",
                name: "{self.name}",
                value: "{self.value}",
                minlength: "{self.minlength}",
                maxlength: "{self.maxlength}",
                min: "{self.min}",
                max: "{self.max}",
                placeholder: "{self.placeholder}",
                required: "{self.required}",
                disabled: "{self.disabled}",
                readonly: "{self.readonly}",
                class: "{class}",
                oninput: move |event| self.oninput.call(event)
            }
        }
    }
}
