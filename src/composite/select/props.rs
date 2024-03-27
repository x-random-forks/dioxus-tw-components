use self::styling::BaseClass;
use crate::*;
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct SelectGroupProps {
    // What will be sent in the request eg name:value where value is represented by the selected SelectItem
    name: String,
    #[props(default = false)]
    required: bool,
    #[props(default = false)]
    disabled: bool,
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    children: Element,
    // Styling
}

impl Component for SelectGroupProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<SelectGroupProps>::Default);
        rsx!(
            select {
                class: "{class}",
                name: "{self.name}",
                oninput: move |e| self.oninput.call(e),
                {self.children}
            }
        )
    }
}

// SelectLabel is used to group SelectItems together under a common label
#[derive(PartialEq, Props, Clone, Component)]
pub struct SelectLabelProps {
    label: String,
    // This disabled the Label and all SelectItems under it
    #[props(default = false)]
    disabled: bool,
    children: Element,
    // Styling
}

impl Component for SelectLabelProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<SelectLabelProps>::Default);
        rsx!(
            optgroup {
                label: "{self.label}",
                disabled: "{self.disabled}",
                class: "{class}",
                {self.children}
            }
        )
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct SelectItemProps {
    // What will be sent in the request eg name:value where value is represented by the selected SelectItem
    value: String,
    // Disabled the SelectItem
    #[props(default = false)]
    disabled: bool,
    // Select the SelectItem by default
    #[props(default = false)]
    selected: bool,
    children: Element,
    // Styling
}

impl Component for SelectItemProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<SelectItemProps>::Default);
        rsx!(
            option {
                value: "{self.value}",
                disabled: "{self.disabled}",
                selected: "{self.selected}",
                class: "{class}",
                {self.children}
            }
        )
    }
}
// SelectTrigger
// SelectContent ?
// SelectGroup ?
// SelectLabel
// SelectItem
