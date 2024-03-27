use self::styling::{BaseClass, Color, Size};
use crate::*;
use component_derive::Component;

pub use Color::{Accent, Primary, Secondary};
pub use Size::{Lg, Md, Sm};

// specifically stylised input type checkbox
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
            label { class: "inline-flex items-center cursor-pointer",
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
                div { class: "{class}" }
                span { class: "ms-3 text-sm font-medium text-gray-900 peer-disabled:text-muted",
                    {self.children}
                }
            }
        )
    }
}
