use dioxus::prelude::*;
use form_builder::prelude::*;
use slugify::slugify;
use std::collections::HashMap;

use crate::{
    atoms::{Button, ButtonVariant},
    form::*,
};

pub trait RenderForm {
    fn render(&self, index: Vec<usize>) -> Element;
}

pub struct FormState {
    user_input: HashMap<String, FieldDataType>,
    form_response: FormResponse,
    form_index: usize,

    // TODO
    pub boool: bool,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            user_input: HashMap::new(),
            form_response: FormResponse::default(),
            form_index: 0,
            boool: false,
        }
    }

    pub fn set_answer(&mut self, answer: FormResponse) {
        self.form_response = answer;
    }

    pub fn set_data_answer(
        &mut self,
        index: usize,
        new_data: FieldDataType,
    ) -> Result<(), DataError> {
        self.form_response.set_data(index, new_data)
    }

    pub fn insert_user_input_value(&mut self, key: String, value: FieldDataType) {
        self.user_input.insert(key, value);
    }

    pub fn insert_checkbox_user_input(&mut self, key: String, value: Vec<String>) {
        self.user_input.insert(key, FieldDataType::Checkbox(value));
    }

    pub fn is_checkbox_empty(&self, key: &str) -> bool {
        if let Some(FieldDataType::Checkbox(values)) = self.user_input.get(key) {
            values.is_empty()
        } else {
            true
        }
    }

    pub fn remove_checkbox_user_input(&mut self, key: String) {
        self.user_input.remove(&key);
    }

    pub fn is_key_present_checkbox(&self, key: &str) -> bool {
        self.user_input.contains_key(key)
    }

    pub fn user_input_to_response(&mut self) {
        for (index, field) in self.user_input.iter() {
            self.form_response
                .set_data(index.parse::<usize>().unwrap(), field.clone())
                .unwrap();
        }
    }

    pub fn get_form_index(&mut self) -> ReadOnlySignal<String> {
        let index = self.form_index.to_string();
        self.form_index += 1;
        log::debug!("Form index {}", self.form_index);
        ReadOnlySignal::new(Signal::new(index))
    }
 
    pub fn toogle(&mut self) {
        self.boool = !self.boool;
    }
}

impl RenderForm for Form {
    fn render(&self, _index: Vec<usize>) -> Element {
        let mut state = use_context_provider(|| Signal::new(FormState::new()));
        // TODO Find why is the form rendered 2 times
        log::debug!("State created");
        state.write().form_index = 0;

        state.write().set_answer(self.new_answer());

        let onsubmit = {
            // let form = self.clone();
            move |_event: FormEvent| {
                log::debug!("{:?}", state.read().user_input);
                // log::debug!("{:#?}", state.read().form_response);

                // state.write().toogle();
                // log::debug!("{}", state.read().boool);

                // state.write().response_to_answer();

                // let is_valid = form.is_valid(&mut state.write().answer.data());
                // log::debug!("Is valid: {}", is_valid);

                // for field in state.read().answer.iter().unwrap() {
                //     if field.error.is_some() {
                //         log::debug!("{:#?}", field.error);
                //     }
                // }
            }
        };

        let rendered_fields: Vec<Element> = self
            .iter()
            .unwrap()
            .map(|(idx, field)| {
                if field.label.is_empty() {
                    return rsx!(  );
                }
                rsx!(
                    { field.render(idx) }
                )
            })
            .collect();

        rsx!(
            Form { class: "group is-form", onsubmit, id: "ex-form",
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

fn convert(v: Option<usize>) -> Option<i64> {
    if let Some(v) = v {
        if v > std::i64::MAX as usize {
            Some(v as i64)
        }
        else {
            None
        }
    } else {
        None
    }
}

impl RenderForm for FormField {
    fn render(&self, index: Vec<usize>) -> Element {
        let mut state = consume_context::<Signal<FormState>>();

        let mut desc = self.label.clone();

        // TODO move this and extend everywhere
        // let required = self.is_required;

        let idx = use_signal(|| index.iter().sum::<usize>() + index.len());
        // let name = use_signal(|| slugify!(&idx.clone().to_string()));
        let name = state.write().get_form_index();

        let content = match self.content.clone() {
            FieldType::Text(text_field) => {
                let value = text_field.default.unwrap_or_default();

                let minlength = convert(text_field.min_length);
                let maxlength = convert(text_field.max_length);

                rsx!(
                    Input {
                        r#type: "text",
                        name,
                        minlength,
                        maxlength,
                        value,
                        oninput: move |event: FormEvent| {
                            state
                                .write()
                                .insert_user_input_value(
                                    name(),
                                    FieldDataType::Text(event.data().value()),
                                );
                        }
                    }
                )
            }
            FieldType::Email(email_field) => {
                let value = email_field.default.clone().unwrap_or("".to_string());

                rsx!(
                    Input {
                        r#type: "email",
                        name,
                        value,
                        oninput: move |event: FormEvent| {
                            state
                                .write()
                                .insert_user_input_value(name(), FieldDataType::Email(event.data().value()));
                            state
                                .write()
                                .set_data_answer(idx(), FieldDataType::Email(event.data().value()))
                                .unwrap();
                        }
                    }
                )
            }
            FieldType::Date(date_field) => {
                let value = date_field.default.clone().unwrap_or("".to_string());

                rsx!(
                    Input {
                        r#type: "date",
                        name,
                        value,
                        oninput: move |event: FormEvent| {
                            state
                                .write()
                                .insert_user_input_value(name(), FieldDataType::Date(event.data().value()));
                        }
                    }
                )
            }
            FieldType::Integer(integer_field) => {
                let value = integer_field.default.clone().unwrap_or(0);
                let min = integer_field.min.unwrap_or(0) as i64;
                let max = integer_field.max.unwrap_or(9999999999) as i64;

                rsx!(
                    Input {
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
                            state.write().insert_user_input_value(name(), FieldDataType::Integer(value));
                        }
                    }
                )
            }
            FieldType::Float(float_field) => {
                let value = float_field.default.clone().unwrap_or(0.0);
                let min = float_field.min.unwrap_or(0.0) as f64;
                let max = float_field.max.unwrap_or(9999999.9) as f64;

                rsx!(
                    Input {
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
                            state.write().insert_user_input_value(name(), FieldDataType::Float(value));
                        }
                    }
                )
            }
            FieldType::TextArea(textarea_field) => {
                let value = textarea_field.default.clone().unwrap_or("".to_string());
                let minlength = textarea_field.min_length.unwrap_or(0) as i64;
                let maxlength = textarea_field.max_length.unwrap_or(9999) as i64;

                rsx!(
                    TextArea {
                        name,
                        minlength,
                        maxlength,
                        value,
                        oninput: move |event: FormEvent| {
                            state
                                .write()
                                .insert_user_input_value(
                                    name(),
                                    FieldDataType::TextArea(event.data().value()),
                                );
                        }
                    }
                )
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
                                        let is_key_present = state.read().is_key_present_checkbox(&idx.to_string());
                                        let event_value = event.data().value();
                                        if !is_key_present {
                                            if event_value == "true" {
                                                state
                                                    .write()
                                                    .insert_checkbox_user_input(
                                                        name(),
                                                        vec![variant.clone().to_string()],
                                                    );
                                            } else {
                                                if let Some(FieldDataType::Checkbox(values)) = state
                                                    .write()
                                                    .user_input
                                                    .get_mut(&name())
                                                {
                                                    values.retain(|x| x != &variant);
                                                }
                                            }
                                        } else {
                                            if event_value == "true" {
                                                if let Some(FieldDataType::Checkbox(values)) = state
                                                    .write()
                                                    .user_input
                                                    .get_mut(&name())
                                                {
                                                    values.push(variant.to_string());
                                                }
                                            } else {
                                                if let Some(FieldDataType::Checkbox(values)) = state
                                                    .write()
                                                    .user_input
                                                    .get_mut(&name())
                                                {
                                                    values.retain(|x| x != &variant);
                                                }
                                                if state.read().is_checkbox_empty(&name()) {
                                                    state.write().remove_checkbox_user_input(name());
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
                                            .insert_user_input_value(name(), FieldDataType::Radio(event.data().value()));
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

                rsx!(
                    Slider {
                        name,
                        min,
                        max,
                        step,
                        oninput: move |event: FormEvent| {
                            let Ok(value) = event.data().value().parse::<i64>() else {
                                log::error!("Error parsing value");
                                return;
                            };
                            state.write().insert_user_input_value(name(), FieldDataType::Slider(value));
                        }
                    }
                )
            }
            FieldType::Combobox(combobox_field) => {
                let variants = combobox_field.variants.clone();

                rsx!(
                    SelectGroup {
                        name,
                        oninput: move |event: FormEvent| {
                            state
                                .write()
                                .insert_user_input_value(
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
                            id: name(),
                            onclick: move |_| {
                                active = !active;
                                state.write().insert_user_input_value(name(), FieldDataType::Toggle(active));
                            }
                        }
                        FormLabel { r#for: name(), {&*desc} }
                    }
                )
            }
            FieldType::Section(_) => {
                let section_label = desc.clone();
                desc.clear();

                let mut list_fields = Vec::new();

                let Ok(iterator) = self.iter(index.clone()) else {
                    return rsx!(  );
                };
                let rendered_fields = iterator
                    .map(|(idx, item)| item.render(idx))
                    .collect::<Vec<Element>>();

                let rendered_fields = rsx!(
                    { rendered_fields.iter() }
                );
                list_fields.push(rendered_fields);

                rsx!(
                    FormList { class: "group sub-form",
                        div { class: "flex space-x-2 place-items-center", FormListTitle { {section_label} } }
                        FormListContent { list_fields }
                    }
                )
            }
            FieldType::List(list_field) => {
                let list_label = desc.clone();
                desc.clear();

                let mut list_fields = Vec::new();

                for _ in 0..list_field.max_length {
                    let Ok(iterator) = self.iter(index.clone()) else {
                        return rsx!(  );
                    };
                    let rendered_fields = iterator
                        .map(|(idx, item)| item.render(idx))
                        .collect::<Vec<Element>>();

                    let rendered_fields = rsx!(
                        { rendered_fields.iter() }
                    );
                    list_fields.push(rendered_fields);
                }

                rsx!(
                    FormList { class: "group sub-form", max_size: list_field.max_length,
                        div { class: "flex space-x-2 place-items-center",
                            FormListTitle { {list_label} }
                            FormListTrigger { plus: false }
                            FormListTrigger { plus: true }
                        }
                        FormListContent { list_fields }
                    }
                )
            }
            _ => {
                rsx!(  )
            }
        };

        rsx!(
            // TODO FORM Component formchild?
            div { class: "relative content-center pt-2 lg:odd:pl-4 lg:odd:border-l-2 group-[.sub-form]:lg:odd:pl-0 group-[.sub-form]:lg:odd:border-none",
                if !desc.is_empty() {
                    FormDesc { class: "", {desc} }
                }
                {content}
            }
        )
    }
}

fn sanitize_string(str1: impl std::fmt::Display, str2: impl std::fmt::Display) -> String {
    slugify!(&format!("{}{}", str1, str2))
}
