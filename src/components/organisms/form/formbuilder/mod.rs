use dioxus::prelude::*;
use form_builder::prelude::*;
use slugify::slugify;
use std::collections::HashMap;

use crate::{
    atoms::{Button, ButtonVariant},
    form::*,
};

pub trait RenderForm {
    fn render(&self, index: usize) -> Element;
}

pub struct FormState {
    response: HashMap<String, FieldDataType>,
    answer: FormResponse,
    pub test_toggle: bool,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            response: HashMap::new(),
            answer: FormResponse::default(),
            test_toggle: false,
        }
    }

    pub fn set_answer(&mut self, answer: FormResponse) {
        self.answer = answer;
    }

    pub fn set_data_answer(
        &mut self,
        index: usize,
        new_data: FieldDataType,
    ) -> Result<(), DataError> {
        self.answer.set_data(index, new_data)
    }

    pub fn insert_response_value(&mut self, key: String, value: FieldDataType) {
        self.response.insert(key, value);
    }

    pub fn insert_checkbox_value(&mut self, key: String, value: Vec<String>) {
        self.response.insert(key, FieldDataType::Checkbox(value));
    }

    pub fn is_checkbox_empty(&self, key: &str) -> bool {
        if let Some(FieldDataType::Checkbox(values)) = self.response.get(key) {
            values.is_empty()
        } else {
            true
        }
    }

    pub fn remove_checkbox(&mut self, key: String) {
        self.response.remove(&key);
    }

    pub fn is_key_present(&self, key: &str) -> bool {
        self.response.contains_key(key)
    }

    pub fn response_to_answer(&mut self) {
        for (index, field) in self.response.iter() {
            self.answer
                .set_data(index.parse::<usize>().unwrap(), field.clone())
                .unwrap();
        }
    }

    pub fn test_toogle(&mut self) {
        self.test_toggle = !self.test_toggle;
    }
}

impl RenderForm for Form {
    fn render(&self, _index: usize) -> Element {
        let mut state = use_context_provider(|| Signal::new(FormState::new()));
        log::debug!("State created");

        state.write().set_answer(self.new_answer());

        let onsubmit = {
            let form = self.clone();
            move |_event: FormEvent| {
                log::debug!("{:#?}", state.read().response);

                state.write().test_toogle();

                state.write().response_to_answer();

                let is_valid = form.is_valid(&mut state.write().answer.data());
                log::debug!("Is valid: {}", is_valid);

                for field in state.read().answer.iter().unwrap() {
                    if field.error.is_some() {
                        log::debug!("{:#?}", field.error);
                    }
                }
            }
        };

        let rendered_fields: Vec<Element> = self
            .iter()
            .unwrap()
            .enumerate()
            .map(|(idx, field)| {
                if field.label.is_empty() {
                    return rsx!();
                }
                rsx!({ field.render(idx) })
            })
            .collect();

        rsx!(
            Form { onsubmit, id: "ex-form",
                FormHeader {
                    h2 { class: "h2", {&*self.label} }
                    if let Some(description) = &self.description {
                        p { class: "paragraph font-medium", {description.clone()} }
                    }
                }
                {rendered_fields.iter()},
                FormFooter {
                    Button {
                        class: "w-full",
                        variant: ButtonVariant::Ghost,
                        r#type: "submit",
                        value: "Submit",
                        "Submit"
                    }
                }
            }
        )
    }
}

impl RenderForm for FormField {
    fn render(&self, index: usize) -> Element {
        let mut state = consume_context::<Signal<FormState>>();

        let mut desc = self.label.clone();

        let index = use_signal(|| index);
        let name = use_signal(|| slugify!(&index.clone().to_string()));

        let content = match self.content.clone() {
            FieldType::Text(text_field) => {
                let value = text_field.default.unwrap_or_default();
                // TODO change this
                let minlength = text_field.min_length.unwrap_or(0) as i64;
                let maxlength = text_field.max_length.unwrap_or(999999) as i64;

                rsx!(Input {
                    r#type: "text",
                    name,
                    minlength,
                    maxlength,
                    value,
                    oninput: move |event: FormEvent| {
                        state.write().insert_response_value(
                            name.to_string(),
                            FieldDataType::Text(event.data().value()),
                        );
                    }
                })
            }
            FieldType::Email(email_field) => {
                let value = email_field.default.clone().unwrap_or("".to_string());

                rsx!(Input {
                    r#type: "email",
                    name,
                    value,
                    oninput: move |event: FormEvent| {
                        state.write().insert_response_value(
                            name(),
                            FieldDataType::Email(event.data().value()),
                        );
                        state
                            .write()
                            .set_data_answer(index(), FieldDataType::Email(event.data().value()))
                            .unwrap();
                    }
                })
            }
            FieldType::Date(date_field) => {
                let value = date_field.default.clone().unwrap_or("".to_string());

                rsx!(Input {
                    r#type: "date",
                    name,
                    value,
                    oninput: move |event: FormEvent| {
                        state.write().insert_response_value(
                            name(),
                            FieldDataType::Date(event.data().value()),
                        );
                    }
                })
            }
            FieldType::Integer(integer_field) => {
                let value = integer_field.default.clone().unwrap_or(0);
                let min = integer_field.min.unwrap_or(0) as i64;
                let max = integer_field.max.unwrap_or(9999999999) as i64;

                rsx!(Input {
                    r#type: "number",
                    name,
                    min,
                    max,
                    value,
                    oninput: move |event: FormEvent| {
                        let Ok(value) = event.data().value().parse::<i64>() else {
                            log::error!("Error parsing value");
                            return;
                        };
                        state
                            .write()
                            .insert_response_value(name(), FieldDataType::Integer(value));
                    }
                })
            }
            FieldType::Float(float_field) => {
                let value = float_field.default.clone().unwrap_or(0.0);
                let min = float_field.min.unwrap_or(0.0) as f64;
                let max = float_field.max.unwrap_or(9999999.9) as f64;

                rsx!(Input {
                    step: "0.001",
                    r#type: "number",
                    name,
                    min,
                    max,
                    value,
                    oninput: move |event: FormEvent| {
                        let Ok(value) = event.data().value().parse::<f64>() else {
                            log::error!("Error parsing value");
                            return;
                        };
                        state
                            .write()
                            .insert_response_value(name(), FieldDataType::Float(value));
                    }
                })
            }
            FieldType::TextArea(textarea_field) => {
                let value = textarea_field.default.clone().unwrap_or("".to_string());
                let minlength = textarea_field.min_length.unwrap_or(0) as i64;
                let maxlength = textarea_field.max_length.unwrap_or(9999) as i64;

                rsx!(TextArea {
                    name,
                    minlength,
                    maxlength,
                    value,
                    oninput: move |event: FormEvent| {
                        state.write().insert_response_value(
                            name(),
                            FieldDataType::TextArea(event.data().value()),
                        );
                    }
                })
            }
            FieldType::Checkbox(checkbox_field) => {
                let variants = checkbox_field.variants.clone();

                rsx!(
                    for variant in variants.into_iter() {
                        div { class: "flex items-center space-x-2",
                            Checkbox {
                                name,
                                id: sanitize_string(name(), &variant),
                                oninput: {
                                    let variant = variant.clone();
                                    move |event: FormEvent| {
                                        let is_key_present = state.read().is_key_present(&index.to_string());
                                        if !is_key_present {
                                            if event.data().value() == "true" {
                                                state
                                                    .write()
                                                    .insert_checkbox_value(
                                                        name(),
                                                        vec![variant.clone().to_string()],
                                                    );
                                            } else {
                                                if let Some(FieldDataType::Checkbox(values)) = state
                                                    .write()
                                                    .response
                                                    .get_mut(&name())
                                                {
                                                    values.retain(|x| x != &variant);
                                                }
                                            }
                                        } else {
                                            if event.data().value() == "true" {
                                                if let Some(FieldDataType::Checkbox(values)) = state
                                                    .write()
                                                    .response
                                                    .get_mut(&name())
                                                {
                                                    values.push(variant.to_string());
                                                }
                                            } else {
                                                if let Some(FieldDataType::Checkbox(values)) = state
                                                    .write()
                                                    .response
                                                    .get_mut(&name())
                                                {
                                                    values.retain(|x| x != &variant);
                                                }
                                                if state.read().is_checkbox_empty(&name()) {
                                                    state.write().remove_checkbox(name());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            FormLabel { r#for: sanitize_string(name(), &variant), {variant} }
                        }
                    }
                )
            }
            FieldType::Radio(radio_field) => {
                let variants = radio_field.variants.clone();

                rsx!(
                    RadioGroup { name: name(),
                        for variant in variants.into_iter() {
                            div { class: "flex items-center space-x-2",
                                RadioItem {
                                    value: variant.clone(),
                                    id: sanitize_string(name(), &*variant.clone()),
                                    name: name(),
                                    oninput: move |event: FormEvent| {
                                        state
                                            .write()
                                            .insert_response_value(name(), FieldDataType::Radio(event.data().value()));
                                    }
                                }
                                FormLabel { r#for: sanitize_string(name(), &*variant.clone()), {&*variant} }
                            }
                        }
                    }
                )
            }
            FieldType::Slider(slider_field) => {
                let min = slider_field.min;
                let max = slider_field.max;
                let step = slider_field.step;

                rsx!(Slider {
                    name,
                    min,
                    max,
                    step,
                    oninput: move |event: FormEvent| {
                        let Ok(value) = event.data().value().parse::<i64>() else {
                            log::error!("Error parsing value");
                            return;
                        };
                        state
                            .write()
                            .insert_response_value(name(), FieldDataType::Slider(value));
                    }
                })
            }
            FieldType::Combobox(combobox_field) => {
                let variants = combobox_field.variants.clone();

                rsx!(
                    SelectGroup {
                        name,
                        oninput: move |event: FormEvent| {
                            state
                                .write()
                                .insert_response_value(
                                    name(),
                                    FieldDataType::Combobox(event.data().value()),
                                );
                        },
                        for variant in variants.into_iter() {
                            SelectItem { value: &*variant, {&*variant} }
                        }
                    }
                )
            }
            FieldType::Toggle(toggle_field) => {
                let mut active = toggle_field.default;

                rsx!(
                    div { class: "flex items-center space-x-2",
                        Toggle {
                            name,
                            active,
                            id:  name(),
                            onclick: move |_| {
                                active = !active;
                                state.write().insert_response_value(name(), FieldDataType::Toggle(active));
                            }
                        }
                        FormLabel { r#for: name(), {&*desc} }
                    }
                )
            }
            FieldType::Section(_) => {
                let section_label = desc.clone();
                desc.clear();

                rsx!(
                    h4 { class: "h4", {section_label} }
                )
            }
            FieldType::List(list_field) => {
                // TODO
                let content = list_field.content.render(0);

                rsx!({ content })
            }
            _ => {
                rsx!()
            }
        };

        rsx!(
            div { class: "relative content-center pt-2 lg:odd:pl-4 lg:odd:border-l-2",
                FormDesc { class: "", {desc} }
                {content}
            }
        )
    }
}

fn sanitize_string(str1: impl std::fmt::Display, str2: impl std::fmt::Display) -> String {
    slugify!(&format!("{}{}", str1, str2))
}
