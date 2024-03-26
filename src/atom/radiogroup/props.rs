use component_derive::Component;

use crate::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct RadioGroupProps {
    name: String,
    #[props(default)]
    default_value: String,
    #[props(default = false)]
    disabled: bool,
    children: Element,
    // Styling
    // Orientation ?
}

impl Component for RadioGroupProps {
    fn view(self) -> Element {
        let class = "flex flex-col";
        rsx!(fieldset {
            class:"{class}",
            {self.children}
        })
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct RadioItemProps {
    // Corresponds to the name of the RadioGroup
    name: String,
    // What will be sent as name:value
    value: String,
    // Is the button checked by default, if several are checked in RadioGroup, only the last one will be checked
    #[props(default = false)]
    checked: bool,
    // Applies to the whole RadioGroup, if true, the form will not be submitted if no RadioItem is checked
    #[props(default = false)]
    required: bool,
    // Disables the radio button
    #[props(default = false)]
    disabled: bool,
    // Styling
}

impl Component for RadioItemProps {
    fn view(self) -> Element {
        let class = "";

        rsx!(input {
            name: "{self.name}",
            value: "{self.value}",
            checked: "{self.checked}",
            required: "{self.required}",
            disabled: "{self.disabled}",
            r#type: "radio",
            class: "{class}",
        })
    }
}
