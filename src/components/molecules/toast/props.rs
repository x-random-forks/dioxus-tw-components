use std::{str::FromStr, string::ToString};

use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::tw_merge;
use web_sys::wasm_bindgen::{closure::Closure, JsValue};

use crate::{
    attributes::{BaseClass, Color, Colorable},
    hooks::{use_element_by_id, use_set_timeout, use_window},
};

/// Used to keep track of the number of toasts created
/// This number is used to create a unique ID in the DOM to then grab it and do whatever with it
/// In this case we use it to remove it after the toast's timeout is up
/// This state also contains the current position of the Toast {} since all
/// the toasts are by default just <li> tag wrapped by the Toast component which is just an <ol> tag
pub struct ToastState {
    toast_count: u64,
    current_position: ToastPosition,
}

impl std::default::Default for ToastState {
    fn default() -> Self {
        ToastState {
            toast_count: 0,
            current_position: ToastPosition::default(),
        }
    }
}

impl ToastState {
    fn increment_toast_count(&mut self) {
        self.toast_count += 1;
    }

    fn decrement_toast_count(&mut self) {
        self.toast_count -= 1;
    }

    fn get_current_position(&self) -> ToastPosition {
        self.current_position
    }

    fn current_position(&mut self, position: ToastPosition) {
        self.current_position = position;
    }

    fn build_toast_id(&self) -> String {
        format!("{}-li-{}", TOAST_ID, self.toast_count)
    }
}

#[derive(Default, Clone, PartialEq, Copy)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    #[default]
    BottomRight,
}

impl FromStr for ToastPosition {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bottomleft" => Ok(ToastPosition::BottomLeft),
            "topleft" => Ok(ToastPosition::TopLeft),
            "topright" => Ok(ToastPosition::TopRight),
            "bottomright" | _ => Ok(ToastPosition::BottomRight),
        }
    }
}

impl ToastPosition {
    /// Will always return a position, if spelling mistake will return ToastPosition::default()
    pub fn str_to_toast_pos<T:ToString>(str: T) -> ToastPosition {
        str.to_string().parse().unwrap()
    }
}

#[derive(Clone, PartialEq)]
pub struct Toast {
    html_tag: &'static str,
    pub(crate) color: Color,
    class: String,
    title: String,
    title_class: String,
    title_tag: Option<String>,
    description: String,
    description_class: String,
    description_tag: Option<String>,
    timeout: Option<i32>,
    // TODO dismiss
    dismiss: bool,
    // TODO Animation: Animation
}

impl std::default::Default for Toast {
    fn default() -> Self {
        Toast {
            html_tag: "li",
            // Set to Color::Muted to construct a default different than Color::Default and still be able to tell the difference when wanting
            // to update the value to Color::Default
            color: Color::Muted,
            class: String::new(),
            title: String::new(),
            title_class: String::new(),
            title_tag: None,
            description: String::new(),
            description_class: String::new(),
            description_tag: None,
            timeout: Some(5000),
            dismiss: true,
        }
    }
}

impl Toast {
    /// Setter on the title
    pub fn title<T: ToString>(mut self, title: T) -> Self {
        self.title = title.to_string();
        self
    }

    /// Used to add classes to the title
    pub fn title_class<T: ToString>(mut self, title_class: T) -> Self {
        self.title_class = title_class.to_string();
        self
    }

    /// Change the default <h6> title's tag
    pub fn title_tag<T: ToString>(mut self, title_tag: T) -> Self {
        self.title_tag = Some(title_tag.to_string());
        self
    }

    /// Setter on the description
    pub fn description<T: ToString>(mut self, description: T) -> Self {
        self.description = description.to_string();
        self
    }

    /// Used to add classes to the description
    pub fn description_class<T: ToString>(mut self, description_class: T) -> Self {
        self.description_class = description_class.to_string();
        self
    }

    /// Change the default <span> description's tag
    pub fn description_tag<T: ToString>(mut self, description_tag: T) -> Self {
        self.description_tag = Some(description_tag.to_string());
        self
    }

    /// Add class to the whole toast
    pub fn class<T: ToString>(mut self, class: T) -> Self {
        self.class = class.to_string();
        self
    }

    /// Change color of the toast (this affects the class attribute)
    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Default delay is 5000 ms, can use None to never close it
    pub fn timeout(mut self, timeout_ms: Option<i32>) -> Self {
        self.timeout = timeout_ms;
        self
    }

    /// WIP does not use for now
    pub fn dismiss(mut self, dismiss: bool) -> Self {
        self.dismiss = dismiss;
        self
    }

    /// Build the whole toast
    fn build(&self, toast_id: &str) -> String {
        let title_tag = &self.title_tag.clone().unwrap_or("h6".to_string());
        let title_class = tw_merge!("h6", &self.title_class);
        let title_class = format!("class=\"{}\"", title_class);
        let title = &self.title;
        let title = format!("<{} {}>{}</{}>", title_tag, title_class, title, title_tag);

        let description_tag = &self.description_tag.clone().unwrap_or("span".to_string());
        let description_class = tw_merge!("span", &self.description_class);
        let description_class = format!("class=\"{}\"", description_class);
        let description = &self.description;
        let description = format!(
            "<{} {}>{}</{}>",
            description_tag, description_class, description, description_tag
        );

        let class = tw_merge!(self.base(), &self.color(), &self.class);
        let class = format!("class=\"{}\"", class);

        let id = format!("id=\"{}\"", toast_id);

        format!(
            "<{} {} {}>{}{}</{}>",
            self.html_tag, id, class, title, description, self.html_tag
        )
    }
}

const TOAST_ID: &str = "dx-toast";

#[props_component(class, children)]
pub fn ToastList() -> Element {
    let class = tw_merge!(
        props.base(),
        &props.class,
        ToastPosition::default().to_string()
    );

    rsx!(
        ol { role: "alert", id: TOAST_ID, class, {children} }
    )
}

/// Changes the position of all the toasts currently active
/// Uses ToastState, if not declared will do nothing
pub fn use_toast_set_position(toast_position: ToastPosition) {
    // TODO use try_consume_context() instead
    let mut state = consume_context::<Signal<ToastState>>();

    if let Ok(element) = use_element_by_id(TOAST_ID) {
        let current_position = state.read().get_current_position();
        if toast_position != current_position {
            // Change <ol> tag class to fit its new position

            let mut class = element.get_attribute("class").unwrap_or_default();

            match class.find(&current_position.to_string()) {
                None => class.push_str(&toast_position.to_string()),
                Some(index) => class.replace_range(
                    index..index + &current_position.to_string().len(),
                    &toast_position.to_string(),
                ),
            };

            state.write().current_position(toast_position);

            element
                .set_attribute("class", &class)
                .unwrap_or_else(|e| log::error!("Failed setting new toast class {:#?}", e));
        }
    }
}

/// Return a string with the id of the newly created toast in the DOM if success, else a web_sys::JsValue Err
/// Will also panic if the function can't find the signal (consume_context())
pub fn use_toast(toast: Toast) -> Result<String, JsValue> {
    // Try getting the Toast in the DOM
    let element = use_element_by_id(TOAST_ID)?;

    // This can panic
    let mut state = consume_context::<Signal<ToastState>>();

    let new_toast_id = state.read().build_toast_id();

    let html = toast.build(&new_toast_id);

    // Insert the newly build toast in the dom
    element.insert_adjacent_html("afterbegin", &html)?;

    // If succeeded to insert it increment toast_count
    state.write().increment_toast_count();

    // Start the toast timeout if there is any
    if let Some(timeout) = toast.timeout {
        begin_toast_timeout(&new_toast_id, timeout, state)?;
    }

    Ok(new_toast_id)
}

/// Return the id of the timeout if success
/// else a Err(web_sys::JsValue)
fn begin_toast_timeout(
    toast_id: &str,
    timeout: i32,
    mut state: Signal<ToastState>,
) -> Result<i32, JsValue> {
    let window = use_window()?;

    let element = use_element_by_id(toast_id).unwrap();

    let closure = Closure::wrap(Box::new(move || {
        element.remove();
        state.write().decrement_toast_count();
    }) as Box<dyn FnMut()>);

    let timeout_result = use_set_timeout(&window, &closure, timeout);

    closure.forget();

    timeout_result
}
