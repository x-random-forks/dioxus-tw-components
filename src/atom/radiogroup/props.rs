use crate::{
    atom::icon::{style::IconSvg, Icon},
    *,
};
use component_derive::Component;

use self::styling::Color;

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

// Struct used to track which RadioItem is currently checked within the RadioGroup
struct RadioGroupSignal(String);

impl Component for RadioGroupProps {
    fn view(self) -> Element {
        use_context_provider(|| Signal::new(RadioGroupSignal(self.default_value)));
        let class = "flex flex-col";
        rsx!(
            div { class: "{class}", {self.children} }
        )
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct RadioItemProps {
    // Corresponds to the name of the RadioGroup
    name: String,
    // What will be sent as name:value
    value: String,
    // Applies to the whole RadioGroup, if true, the form will not be submitted if no RadioItem is checked
    #[props(default = false)]
    required: bool,
    // Disable the radio button
    #[props(default = false)]
    disabled: bool,
    // Callback when the RadioItem is checked
    #[props(default)]
    oninput: EventHandler<FormEvent>,
    children: Element,
    // Styling
    #[props(default)]
    color: Color,
}

impl Component for RadioItemProps {
    fn view(self) -> Element {
        let mut parent_context = consume_context::<Signal<RadioGroupSignal>>();

        // TODO Should do both at the same time
        let svg = if parent_context.read().0 == self.value {
            IconSvg::CircleInnerCircle
        } else {
            IconSvg::HollowCircle
        };

        let checked = if parent_context.read().0 == self.value {
            true
        } else {
            false
        };

        // TODO Will get rid of this when the last div is removed
        let text_color = match self.color {
            Color::Default => "text-foreground",
            Color::Primary => "text-primary",
            Color::Secondary => "text-secondary",
            Color::Accent => "text-accent",
            _ => "text-none",
        };

        rsx!(
            label { class: "{self.name}",
                div { class: "flex items-center",
                    input {
                        name: "{self.name}",
                        value: "{self.value}",
                        checked: "{checked}",
                        required: "{self.required}",
                        disabled: "{self.disabled}",
                        r#type: "radio",
                        oninput: move |e| {
                            parent_context.set(RadioGroupSignal(self.value.clone()));
                            self.oninput.call(e);
                        },
                        class: "hidden"
                    }
                    div { class: "size-4", Icon { svg: svg, color: self.color } }
                    // TODO Move this into another comp
                    div { class: "{text_color} font-medium", {self.children} }
                }
            }
        )
    }
}
