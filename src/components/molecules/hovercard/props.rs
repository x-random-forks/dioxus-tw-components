use crate::{
    attributes::*,
    hooks::{use_document, use_window},
};
use chrono::{DateTime, Local, TimeDelta};
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use dioxus_core::AttributeValue;
use dioxus_elements::geometry::euclid::Rect;
use gloo_timers::future::TimeoutFuture;

#[derive(Clone, Debug)]
pub struct HoverState {
    is_active: bool,
    is_hovered: bool,
    last_hover: DateTime<Local>,
    closing_delay_ms: TimeDelta,
}

impl HoverState {
    fn new(closing_delay_ms: u32) -> Self {
        Self {
            is_active: false,
            closing_delay_ms: TimeDelta::milliseconds(closing_delay_ms as i64),
            is_hovered: false,
            last_hover: DateTime::default(),
        }
    }

    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn open(&mut self) {
        self.is_active = true;
    }

    fn close(&mut self) {
        self.is_active = false;
    }

    fn set_is_hovered(&mut self, is_hovered: bool) {
        self.is_hovered = is_hovered;
    }

    fn get_is_hovered(&self) -> bool {
        self.is_hovered
    }

    fn set_last_hover(&mut self, last_hover: DateTime<Local>) {
        self.last_hover = last_hover;
    }

    fn get_last_hover(&self) -> DateTime<Local> {
        self.last_hover
    }

    fn get_closing_delay(&self) -> TimeDelta {
        self.closing_delay_ms
    }
}

impl IntoAttributeValue for HoverState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct HoverCardProps {
    /// Corresponds to the time in ms it takes for the toggle to close itself if not hovered
    #[props(default = 500)]
    closing_delay_ms: u32,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for HoverCardProps {
    fn default() -> Self {
        Self {
            closing_delay_ms: 500,
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

pub fn HoverCard(mut props: HoverCardProps) -> Element {
    let mut state = use_context_provider(|| Signal::new(HoverState::new(props.closing_delay_ms)));

    props.update_class_attribute();

    let onmouseenter = move |_event| {
        state.write().set_is_hovered(true);
        state.write().open();
    };

    let onmouseleave = move |_| {
        state.write().set_last_hover(Local::now());
        state.write().set_is_hovered(false);

        let closing_delay_ms = state.read().closing_delay_ms;

        spawn(async move {
            TimeoutFuture::new(
                closing_delay_ms
                    .num_milliseconds()
                    .try_into()
                    .unwrap_or_default(),
            )
            .await;

            let is_hovered = state.read().get_is_hovered();

            let last_hover = state.read().get_last_hover();
            let now = Local::now();
            let dt = state.read().get_closing_delay();

            if !is_hovered && now - last_hover >= dt {
                state.write().close();
            }
        });
    };

    rsx!(
        div {
            "data-state": state.into_value(),
            onmouseenter,
            onmouseleave,
            ..props.attributes,
            {props.children}
        }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct HoverCardTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for HoverCardTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

pub fn HoverCardTrigger(mut props: HoverCardTriggerProps) -> Element {
    let mut state = use_context::<Signal<HoverState>>();

    props.update_class_attribute();

    // We need this event here to not close the hover card when clicking its content
    let onclick = move |event| {
        state.write().toggle();
        props.onclick.call(event);
    };

    rsx!(
        div {
            role: "button",
            "data-state": state.into_value(),
            onclick,
            ..props.attributes,
            {props.children}
        }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct HoverCardContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,

    #[props(default = true)]
    pub apply_position: bool,

    children: Element,
}

impl std::default::Default for HoverCardContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            apply_position: true,
            children: rsx! {},
        }
    }
}

pub fn HoverCardContent(mut props: HoverCardContentProps) -> Element {
    let state = use_context::<Signal<HoverState>>();

    props.update_class_attribute();

    let mut body_width = use_signal(|| 0);

    // Get the body width
    use_effect(move || {
        if let Ok(window) = use_window() {
            let document = use_document(&window).unwrap();
            let a = document.body().unwrap();
            body_width.set(a.client_width())
        }
    });

    let mut rect_sig = use_signal(Rect::default);

    // Retrieve the hover content rect
    let onmounted = move |event: MountedEvent| async move {
        let rect = event.get_client_rect().await;
        rect_sig.set(rect.unwrap());
    };

    // Css property to position the hover content
    let mut position = use_signal(|| String::from(""));

    use_effect(move || {
        let rect = rect_sig();
        let body_width = body_width();

        // Place the content to fit inside the screen
        if rect.origin.x < 0. {
            position.set("left: 0px;".to_string())
        } else if (rect.origin.x + rect.size.width) as u32 as i32 > body_width {
            position.set("right: 0px;".to_string())
        } else {
            position.set("left: 50%; translate: -50%;".to_string())
        }
    });

    rsx!(
        div {
            style: if props.apply_position { position },
            "data-state": state.into_value(),
            onmounted,
            ..props.attributes,
            {props.children}
        }
    )
}
