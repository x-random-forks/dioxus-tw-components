use std::time::Duration;

use dioxus::prelude::*;
use gloo_timers::{callback::Timeout, future::{sleep, TimeoutFuture}};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn HoverCardPage() -> Element {
    rsx!(
        HoverCard { 
            HoverCardTrigger { "Hover" }
            HoverCardContent { 
                div { class: "border", "CONTENT" }
            }
        }
    )
}

pub struct HoverState {
    is_open: bool,
    timeout: Option<TimeoutFuture>
}

impl HoverState {
    fn new() -> Self {
        Self { is_open: false, timeout: None }
    }

    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    fn open(&mut self) {
        self.is_open = true;
    }

    fn close(&mut self) {
        self.is_open = false;
    }

    fn set_timeout(&mut self, timeout: TimeoutFuture) {
        self.timeout = Some(timeout);
    }
}

#[component]
pub fn HoverCard(children: Element) -> Element {
    let _state = use_context_provider(|| Signal::new(HoverState::new()));

    rsx!(
        div { {children} }
    )
}

#[component]
pub fn HoverCardTrigger(children: Element) -> Element {
    // let state = try_use_context::Signal<HoverState>>();
    let mut state: Signal<HoverState> = use_context();

    rsx!(
        div {
            onmouseenter: move |_| {
                let is_open = state.read().is_open;
                if !is_open {
                    spawn(async move {
                        TimeoutFuture::new(1000).await;
                        state.write().open();
                    });
                } else {
                    state.write().open();
                }
            },
            onmouseleave: move |_| {
                spawn(async move {
                    state.write().set_timeout(sleep(Duration::from_secs(1)));
                    log::debug!("There");
                    state.write().close();
                });
            },
            {children}
        }
    )
}

#[component]
pub fn HoverCardContent(children: Element) -> Element {
    let mut state: Signal<HoverState> = use_context();

    let hidden = use_memo(move || {
        !state.read().is_open
    });

    rsx!(
        div { hidden, {children} }
    )
}
