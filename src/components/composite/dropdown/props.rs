use dioxus::{
    html::geometry::euclid::{Point2D, Rect},
    prelude::*,
};
use props_component_macro::{props_component, DataState, HasStateAttr};
use tailwind_fuse::*;
use web_sys::wasm_bindgen::closure::Closure;

use crate::{
    attributes::*,
    hooks::{use_clear_timeout_id, use_set_timeout, use_window},
    LibState,
};

#[derive(Clone, Copy, DataState)]
struct DropdownState {
    state_attr_value: DataStateAttrValue,
    timeout_id: i32,
    closing_delay_ms: i32,
    trigger_rect: Rect<f64, f64>,
    content_rect: Rect<f64, f64>,
}

impl DropdownState {
    fn new(closing_delay_ms: i32) -> Self {
        Self {
            state_attr_value: DataStateAttrValue::Inactive,
            timeout_id: -1,
            closing_delay_ms,
            trigger_rect: Rect::zero(),
            content_rect: Rect::zero(),
        }
    }

    fn toggle(&mut self) {
        self.state_attr_value = -self.state_attr_value;
    }

    fn close(&mut self) {
        self.state_attr_value = DataStateAttrValue::Inactive;
    }

    fn set_timeout_id(&mut self, id: i32) {
        self.timeout_id = id;
    }

    fn get_timeout_id(&self) -> i32 {
        self.timeout_id
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

    fn get_closing_delay(&self) -> i32 {
        self.closing_delay_ms
    }
}

/// Usage:
/// ```ignore
/// Dropdown {
///    DropdownToggle {
///       Button { "Dropdown" } // Can be anything like a div, button, etc
///     }
///     DropdownContent {
///       div { "content" }
///    }
/// }
/// ```
#[props_component(class, id, children)]
#[derive(HasStateAttr)]
pub fn Dropdown(
    #[props(extends = div)] mut attributes: Vec<Attribute>,
    /// Correponds to the time in ms it takes for the toggle to close itself if not active, -1 disabled this feature (default)
    #[props(default = -1)]
    closing_delay_ms: i32,
) -> Element {
    let class = tw_merge!(props.base(), &props.class);

    let state = use_context_provider(|| Signal::new(DropdownState::new(props.closing_delay_ms)));

    props.add_datastate_to_attr(state);

    rsx!(
        div { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(children)]
#[derive(HasStateAttr)]
pub fn DropdownToggle(#[props(extends = div)] mut attributes: Vec<Attribute>) -> Element {
    // Use an "useless div" to wrap the dropdown toggle and get the onclick event, so the user can put
    // Everything inside the DropdownToggle not just only a button
    let class = "inline-block";

    let mut state = consume_context::<Signal<DropdownState>>();

    let onmounted = move |event: Event<MountedData>| async move {
        match event.get_client_rect().await {
            Ok(rect) => state.write().set_toggle_rect(rect),
            Err(err) => log::error!("{:?}", err),
        }
    };

    let onclick = move |_: MouseEvent| {
        state.write().toggle();

        // Remove the timeout if the dropdown is closed manually
        let is_active = state.read().is_active();

        if is_active {
            match use_window() {
                Ok(ref window) => use_clear_timeout_id(window, state.read().get_timeout_id()),
                Err(err) => log::error!("{:?}", err),
            };
        }
    };

    let onmouseleave = move |_| {
        let closing_delay = state.read().get_closing_delay();
        let is_active = state.read().is_active();

        if is_active && closing_delay > 0 {
            begin_timeout(state)
        }
    };

    props.add_datastate_to_attr(state);

    rsx!(
        div {
            ..props.attributes,
            class: class,
            onmounted: onmounted,
            onclick: onclick,
            onmouseleave: onmouseleave,
            { props.children }
        }
    )
}

#[props_component(class, id, children)]
#[derive(HasStateAttr)]
pub fn DropdownContent(
    #[props(extends = div)] mut attributes: Vec<Attribute>,
    #[props(default)] animation: Animation,
) -> Element {
    let class = tw_merge!(props.base(), props.animation(), &props.class);

    let mut state = consume_context::<Signal<DropdownState>>();

    let onmounted = move |event: Event<MountedData>| async move {
        match event.get_client_rect().await {
            Ok(rect) => state.write().set_content_rect(rect),
            Err(err) => log::error!("{:?}", err),
        }
    };

    let onmouseleave = move |_| {
        let closing_delay = state.read().get_closing_delay();

        if closing_delay > 0 {
            begin_timeout(state)
        }
    };

    let onmouseenter = move |_| {
        match use_window() {
            Ok(ref window) => use_clear_timeout_id(window, state.read().get_timeout_id()),
            Err(err) => log::error!("{:?}", err),
        };
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
        let is_active = state.read().is_active();

        if is_active && !rect_toggle.contains(click) && !rect_content.contains(click) {
            state.write().close();
        }
    });

    props.add_datastate_to_attr(state);

    rsx!(
        div {
            ..props.attributes,
            class: class,
            id: props.id,
            onmounted: onmounted,
            onmouseleave: onmouseleave,
            onmouseenter: onmouseenter,
            {props.children}
        }
    )
}

fn begin_timeout(mut state: Signal<DropdownState>) {
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

    let close_delay_duration_ms = state.read().get_closing_delay();

    if let Ok(id) = use_set_timeout(&window, &closure, close_delay_duration_ms) {
        state.write().set_timeout_id(id);
    }

    closure.forget();
}
