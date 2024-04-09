use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct SelectGroupProps {
    #[props(default)]
    name: String,
    // TODO Currently not woking
    #[props(default = false)]
    required: bool,
    #[props(default = false)]
    disabled: bool,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for SelectGroupProps {
    fn view(self) -> Element {
        let class = SelectGroupClass::builder().with_class(self.class);

        rsx!(
            select {
                name: "{self.name}",
                required: "{self.required}",
                disabled: "{self.disabled}",
                class: "{class}",
                oninput: move |e| self.oninput.call(e),
                {self.children}
            }
        )
    }
}

// What will be shown by default in the SelectGroup when nothing is selected yet
#[derive(PartialEq, Props, Clone, Component)]
pub struct SelectPlaceholderProps {
    children: Element,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for SelectPlaceholderProps {
    fn view(self) -> Element {
        let class = SelectPlaceholderClass::builder().with_class(self.class);

        rsx!(
            option { disabled: true, selected: true, value: "", class: "{class}", {self.children} }
        )
    }
}

// SelectLabel is used to group SelectItems together under a common label
#[derive(PartialEq, Props, Clone, Component)]
pub struct SelectLabelProps {
    #[props(default)]
    label: String,
    #[props(default = false)]
    disabled: bool,

    children: Element,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for SelectLabelProps {
    fn view(self) -> Element {
        let class = SelectLabelClass::builder().with_class(self.class);

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
    #[props(default)]
    value: String,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    selected: bool,

    children: Element,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for SelectItemProps {
    fn view(self) -> Element {
        let class = SelectItemClass::builder().with_class(self.class);

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
