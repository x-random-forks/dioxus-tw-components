use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct RadioProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional, default = false)]
    checked: bool,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub size: ReadOnlySignal<Size>,
}

impl std::default::Default for RadioProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            checked: false,
            oninput: EventHandler::<FormEvent>::default(),
            color: ReadOnlySignal::<Color>::default(),
            size: ReadOnlySignal::<Size>::default(),
        }
    }
}

pub fn Radio(mut props: RadioProps) -> Element {
    props.update_class_attribute();

    let oninput = move |event| props.oninput.call(event);

    rsx!(
        input {
            r#type: "radio",
            checked: props.checked,
            oninput,
            ..props.attributes,
        }
    )
}
