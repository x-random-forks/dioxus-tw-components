use dioxus::{dioxus_core::AttributeValue, prelude::*};
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

#[props_component(id, class)]
pub fn Input(
    #[props(extends = input)] mut attributes: Vec<Attribute>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
    #[props(default)] size: Size,
    #[props(default)] color: Color,
    // TODO WIP
    // #[props(default = false)] show_tips: bool,
) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.size(), &props.class);

    let oninput = move |event| {
        if let Some(oc) = &props.oninput {
            oc.call(event)
        }
    };

    // TODO WIP
    // let tips = props.generate_default_tips();
    // if let Some(state) = try_consume_context::<Signal<FormState>>() {
    //     match state.read().boool {
    //         true => props
    //             .attributes
    //             .push(DataAttr::Valid.to_attr(DataValidValue::True)),
    //         false => props
    //             .attributes
    //             .push(DataAttr::Valid.to_attr(DataValidValue::False)),
    //     };
    // }

    rsx! {
        input {
            ..props.attributes,
            oninput: oninput,
            class,
            id: props.id
        }
    }
}
// for tip in tips {
//     {tip}
// }
// span { class: "ml-2 hidden peer-data-[valid=true]:block text-success", "Validation test success" }
// span { class: "ml-2 hidden peer-data-[valid=false]:block text-destructive",
//     "Validation test failure"
// }

impl TipsFormat for InputProps {
    fn tips_format(&self) -> &str {
        "ml-2"
    }
}

impl DefaultTips for InputProps {
    fn generate_default_tips(&self) -> Vec<Element> {
        let vec = self
            .attributes
            .iter()
            .fold(Vec::<Element>::new(), |mut acc, attr| {
                acc.append(
                    &mut FormError::variants()
                        .iter()
                        .map(|tips| {
                            if tips.to_string() == attr.name {
                                tips.generate_rsx(attr.value.clone(), self.tips_format())
                            } else {
                                rsx!(  )
                            }
                        })
                        .collect::<Vec<Element>>(),
                );
                acc
            });
        vec
    }
}

pub enum FormError {
    MaxLength,
    MinLength,
    Required,
    // TODO Custom
}

impl FormError {
    pub fn variants() -> Vec<FormError> {
        vec![
            FormError::MinLength,
            FormError::MaxLength,
            FormError::Required,
        ]
    }

    // TODO need to return an Option<Element> there to avoid having tons of <pre> tags in browsers
    pub fn generate_rsx(&self, value: AttributeValue, tips_format: &str) -> Element {
        match self {
            FormError::MaxLength => {
                let AttributeValue::Int(value) = value else {
                    return rsx!(  );
                };
                if value == 9999 {
                    return rsx!(  );
                } else {
                    let content = format!("Max Length = {value}");

                    rsx!(
                        div { class: tips_format, {content} }
                    )
                }
            }
            FormError::MinLength => {
                let AttributeValue::Int(value) = value else {
                    return rsx!(  );
                };
                if value == 0 {
                    return rsx!(  );
                } else {
                    let content = format!("Min Length = {value}");

                    rsx!(
                        div { class: tips_format, {content} }
                    )
                }
            }
            FormError::Required => {
                rsx!(
                    div { class: tips_format, { "This field is required" } }
                )
            }
        }
    }
}

impl std::fmt::Display for FormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormError::MinLength => write!(f, "minlength"),
            FormError::MaxLength => write!(f, "maxlength"),
            FormError::Required => write!(f, "required"),
        }
    }
}

pub trait DefaultTips: TipsFormat {
    fn generate_default_tips(&self) -> Vec<Element>;
}

pub trait TipsFormat {
    fn tips_format(&self) -> &str;
}

// fn MyTips() -> Element {
//     let class = "hidden peer-data-[valid=false]:block peer-data-[valid=false]:text-destructive";

//     rsx!(
//         span { class, "minlen" }
//     )
// }

// pub trait DefaultFormTips {
//     fn tips(&self, tips: impl RenderFormTips) -> Element;
// }

// impl DefaultFormTips for InputProps {
//     fn tips(&self, tips: impl RenderFormTips) -> Element {
//         rsx!()
//     }
// }

// pub trait RenderFormTips {
//     fn render_tips(&self) -> Element;
// }

// pub struct Tips {
//     tips: Vec<Tip>,
// }

// impl Tips {
//     fn default() -> Self {
//         Tips { tips: Vec::new() }
//     }

//     fn push(mut self, tip: Tip) -> Self {
//         self.tips.push(tip);
//         self
//     }
// }

// pub struct Tip {
//     state: DataValidValue,
//     rendered: Element,
// }

// impl Tip {
//     fn render() -> Element {
//         rsx!()
//     }
// }

pub enum DataAttr {
    Active,
    Valid,
}

impl DataAttr {
    fn to_attr(self, value: impl IntoAttributeValue) -> Attribute {
        match self {
            DataAttr::Active => Attribute::new("data-active", value, None, true),
            DataAttr::Valid => Attribute::new("data-valid", value, None, true),
        }
    }

    fn to_style_attr(self, value: DataValidValue) -> String {
        let attr = match self {
            DataAttr::Active => "active".to_string(),
            DataAttr::Valid => "valid".to_string(),
        };
        match value {
            DataValidValue::Valid => format!("data-[{attr}={}]:", value.to_string()),
            DataValidValue::Invalid => format!("data-[{attr}={}]:", value.to_string()),
            DataValidValue::Hidden => format!("data-[{attr}={}]:", value.to_string()),
            DataValidValue::Always | _ => format!(""),
        }
    }
}

#[derive(Clone)]
pub enum DataValidValue {
    // Used by the props
    True,
    False,
    Inactive,
    // Used for the tips
    Always,
    Invalid,
    Valid,
    Hidden,
}

impl std::fmt::Display for DataValidValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataValidValue::True => "true",
                DataValidValue::False => "false",
                DataValidValue::Inactive => "inactive",
                DataValidValue::Always => "always",
                DataValidValue::Invalid => "invalid",
                DataValidValue::Valid => "valid",
                DataValidValue::Hidden => "hidden",
            }
        )
    }
}

impl IntoAttributeValue for DataValidValue {
    fn into_value(self) -> dioxus_core::AttributeValue {
        dioxus::prelude::dioxus_core::AttributeValue::Text(self.to_string())
    }
}
