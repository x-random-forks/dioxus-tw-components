use dioxus::{
    html::geometry::euclid::{Point2D, Rect},
    prelude::*,
};
use props_component_macro::props_component;
use tailwind_fuse::*;
use web_sys::wasm_bindgen::closure::Closure;

use crate::{
    attributes::*,
    hooks::{use_clear_timeout_id, use_set_timeout, use_window},
    LibState,
};

#[derive(Clone, Copy)]
struct DropdownState {
    state_attribute: StateAttribute,
    timeout_id: i32,
    trigger_rect: Rect<f64, f64>,
    content_rect: Rect<f64, f64>,
}

impl DropdownState {
    fn new() -> Self {
        Self {
            state_attribute: StateAttribute::Inactive,
            timeout_id: -1,
            trigger_rect: Rect::zero(),
            content_rect: Rect::zero(),
        }
    }

    fn toggle(&mut self) {
        self.state_attribute = match self.state_attribute {
            StateAttribute::Active => StateAttribute::Inactive,
            StateAttribute::Inactive => StateAttribute::Active,
        };
    }

    fn get_state_attribute(&self) -> StateAttribute {
        self.state_attribute
    }

    fn close(&mut self) {
        self.state_attribute = StateAttribute::Inactive;
    }

    fn set_timeout_id(&mut self, id: i32) {
        self.timeout_id = id;
    }

    fn set_toggle_rect(&mut self, rect: Rect<f64, f64>) {
        self.trigger_rect = rect;
    }

    fn set_content_rect(&mut self, rect: Rect<f64, f64>) {
        self.content_rect = rect;
    }

    fn get_toggle_rect(&self) -> Rect<f64, f64> {
        self.trigger_rect
    }

    fn get_content_rect(&self) -> Rect<f64, f64> {
        self.content_rect
    }
}

impl IntoAttributeValue for DropdownState {
    fn into_value(self) -> dioxus::prelude::dioxus_core::AttributeValue {
        self.get_state_attribute().into_value()
    }
}

#[props_component(class, id, children)]
pub fn Dropdown() -> Element {
    let class = tw_merge!(props.base(), props.class);

    use_context_provider(|| Signal::new(DropdownState::new()));

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

#[props_component(children)]
pub fn DropdownToggle(#[props(extends = div)] attributes: Vec<Attribute>) -> Element {
    // Use an "useless div" to wrap the dropdown toggle and get the onclick event, so the user can put
    // Everything inside the DropdownToggle not just only a button
    let class = "inline-block";

    let mut state = consume_context::<Signal<DropdownState>>();

    let onmounted = move |event: Event<MountedData>| async move {
        // TODO remove unwrap
        state
            .write()
            .set_toggle_rect(event.get_client_rect().await.unwrap());
    };

    let onclick = move |_: MouseEvent| {
        state.write().toggle();
        // Remove the timeout if the dropdown is closed manually
        // REVIEW error handling ?
        if state.read().get_state_attribute().is_active() {
            let window = match use_window() {
                Ok(window) => window,
                Err(err) => {
                    log::error!("{:?}", err);
                    return;
                }
            };
            use_clear_timeout_id(&window, state.read().timeout_id);
        }
    };

    // Convert the state to an AttributeValue
    let data_state = state.read().get_state_attribute().into_value();

    // Temporary, I would like to be able to pass the new_attribute directly to the rsx! macro but idk if it's possible
    // REVIEW, add this to macro props_component if not possible ?
    let new_attribute = Attribute::new("data-state", data_state, None, true);
    let mut attr = props.attributes.clone();
    attr.push(new_attribute);

    rsx!(
        div { ..attr, class, onmounted, onclick, { props.children } }
    )
}

#[props_component(class, id, children)]
pub fn DropdownContent() -> Element {
    let class = tw_merge!(props.base(), props.class);

    let mut state = consume_context::<Signal<DropdownState>>();

    let onmounted = move |event: Event<MountedData>| async move {
        // TODO remove unwrap
        state
            .write()
            .set_content_rect(event.get_client_rect().await.unwrap());
    };

    let onmouseleave = move |_| {
        let window = match use_window() {
            Ok(window) => window,
            Err(err) => {
                log::error!("{:?}", err);
                return;
            }
        };

        let closure = Closure::wrap(Box::new(move || {
            state.write().close();
        }) as Box<dyn FnMut()>);

        if let Ok(id) = use_set_timeout(&window, &closure, 1000) {
            state.write().set_timeout_id(id);
        }

        closure.forget();
    };

    let onmouseenter = move |_| {
        let window = match use_window() {
            Ok(window) => window,
            Err(err) => {
                log::error!("{:?}", err);
                return;
            }
        };

        use_clear_timeout_id(&window, state.read().timeout_id);
    };
    let app_state = consume_context::<Signal<LibState>>();

    use_memo(move || {
        let click: Point2D<f64, f64> = app_state
            .read()
            .get_last_click_coordinates()
            .client()
            .cast_unit();

        let rect_toggle = state.read().get_toggle_rect();
        let rect_content = state.read().get_content_rect();

        if state.read().get_state_attribute().is_active()
            && !rect_toggle.contains(click)
            && !rect_content.contains(click)
        {
            state.write().close();
        }
    });

    rsx!(
        div { "data-state": state.read().into_value(), class, id: props.id, onmounted, onmouseleave, onmouseenter, {props.children} }
    )
}
