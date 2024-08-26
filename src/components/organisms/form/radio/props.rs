use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
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

pub fn Radio(mut props: RadioProps) -> Element {
    props.update_class_attribute();

    let oninput = move |event| props.oninput.call(event);

    rsx!(
        input {
            ..props.attributes,
            r#type: "radio",
            checked: props.checked,
            oninput
        }
    )
}
