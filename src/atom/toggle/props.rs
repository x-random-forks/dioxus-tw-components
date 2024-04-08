use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

// Specifically stylised input type checkbox
// The input use the tailwind peer class, you can use at your advantage to style the children
// eg peer-disabled:font-mute will change children text-color when the input is disabled (Label component already does this by default)
#[derive(PartialEq, Props, Clone, Component)]
pub struct ToggleProps {
    name: String,
    value: String,
    #[props(default = false)]
    checked: bool,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    required: bool,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    children: Element,

    // Styling
    #[props(default)]
    color: ToggleColor,
    #[props(default)]
    size: ToggleSize,
    #[props(default)]
    class: String,
}

impl Component for ToggleProps {
    fn view(self) -> Element {
        let class = ToggleClass::builder()
            .color(self.color)
            .size(self.size)
            .with_class(self.class);

        rsx!(
            // Label that wraps the input and the toggle switch so the user can click on the switch or the children to interact with the input
            label { class: "peer flex items-center cursor-pointer gap-x-2",
                input {
                    name: "{self.name}",
                    value: "{self.value}",
                    r#type: "checkbox",
                    checked: "{self.checked}",
                    disabled: "{self.disabled}",
                    required: "{self.required}",
                    // Set this input to be hidden except for screen readers
                    // We a custom visual toggle so we don't need the default input
                    class: "sr-only peer",
                    oninput: move |e| self.oninput.call(e)
                }
                // Div that renders the toggle switch
                div { class: "{class}" }
                div { class: "peer-disabled:opacity-50", {self.children} }
            }
        )
    }
}
