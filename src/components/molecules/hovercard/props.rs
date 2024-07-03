use crate::attributes::*;
use chrono::{DateTime, Local, TimeDelta};
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use dioxus_core::AttributeValue;
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

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct HoverCardProps {
    /// Corresponds to the time in ms it takes for the toggle to close itself if not hovered
    #[props(default = 500)]
    closing_delay_ms: u32,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
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
        div { ..props.attributes, "data-state": state.into_value(), onmouseenter, onmouseleave, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct HoverCardTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn HoverCardTrigger(mut props: HoverCardTriggerProps) -> Element {
    let mut state = use_context::<Signal<HoverState>>();

    props.update_class_attribute();

    // We need this event here to not close the hover card when clicking its content
    let onclick = move |_| {
        state.write().toggle();
    };

    rsx!(
        div {
            ..props.attributes,
            role: "button",
            "data-state": state.into_value(),
            onclick,
            {props.children}
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct HoverCardContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

pub fn HoverCardContent(mut props: HoverCardContentProps) -> Element {
    let state = use_context::<Signal<HoverState>>();

    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, "data-state": state.into_value(), {props.children} }
    )
}
