use self::styling::{BaseClass, Color, Size};
use crate::*;
use component_derive::Component;

pub use Color::{Accent, Primary, Secondary};
pub use Size::{Lg, Md, Sm};

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
    children: Element,
    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    // Styling
    #[props(default = Color::Primary)]
    color: Color<ToggleProps>,
    #[props(default = Size::Md)]
    size: Size<ToggleProps>,
}

impl Component for ToggleProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<ToggleProps>::Default, self.color, self.size);
        rsx!(
            // Label that wraps the input and the toggle switch so the user can click on the switch or the children to interact with the input
            label { class: "flex items-center cursor-pointer gap-x-2",
                input {
                    name: "{self.name}",
                    value: "{self.value}",
                    r#type: "checkbox",
                    checked: "{self.checked}",
                    disabled: "{self.disabled}",
                    required: "{self.required}",
                    // Set this input to be hidden except for screen readers
                    class: "sr-only peer",
                    oninput: move |e| self.oninput.call(e)
                }
                // Div that renders the toggle switch
                div { class: "{class}" }
                {self.children}
            }
        )
    }
}
