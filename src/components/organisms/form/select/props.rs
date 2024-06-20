use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SelectGroupProps {
    #[props(extends = select, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    children: Element,
}

pub fn SelectGroup(mut props: SelectGroupProps) -> Element {
    props.update_class_attribute();

    let oninput = move |event| props.oninput.call(event);

    rsx!(
        select { ..props.attributes, oninput: oninput, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SelectPlaceholderProps {
    #[props(extends = option, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn SelectPlaceholder(mut props: SelectPlaceholderProps) -> Element {
    props.update_class_attribute();

    rsx!(
        option { disabled: true, selected: true, value: r#"{""}"#, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SelectLabelProps {
    #[props(extends = optgroup, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

pub fn SelectLabel(mut props: SelectLabelProps) -> Element {
    props.update_class_attribute();

    rsx!(
        optgroup { ..props.attributes }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SelectItemProps {
    #[props(extends = option, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default = None)]
    selected: Option<bool>,

    children: Element,
}

pub fn SelectItem(mut props: SelectItemProps) -> Element {
    props.update_class_attribute();

    if let Some(selected) = props.selected {
        rsx!(
            option { ..props.attributes, selected, {props.children} }
        )
    } else {
        rsx!(
            option { ..props.attributes, {props.children} }
        )
    }
}
