use crate::{attributes::*, hooks::*};
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use gloo_timers::future::TimeoutFuture;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct ToasterProps {
    #[props(extends = ol, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// The toaster must wrap around your App as high as possible to be used
pub fn Toaster(mut props: ToasterProps) -> Element {
    props.update_class_attribute();

    let state =
        use_context_provider::<Signal<ToasterState>>(|| Signal::new(ToasterState::default()));

    rsx!(
        {props.children},
        ol { role: "alert", id: "dx-toast", ..props.attributes,
            if state.read().toasts.len() > 0 {
                for index in 0..state.read().toasts.len() {
                    ToastRenderer { state, toast: state.map(move |state| &state.toasts[index]) }
                }
            }
        }
    )
}

/// Used to keep track of all the current toasts, for now it only keeps 1 Toast
pub struct ToasterState {
    toasts: Vec<Toast>,
}

impl std::default::Default for ToasterState {
    fn default() -> Self {
        ToasterState { toasts: vec![] }
    }
}

/// A Toast with a default duration of 10s
#[derive(Clone, Debug, PartialEq, UiComp)]
pub struct Toast {
    id: String,
    title: String,
    description: Element,
    duration_in_ms: u32,
    is_closable: bool,
    pub color: Color,
    pub animation: Animation,
    state: ToastState,
}

impl std::default::Default for Toast {
    fn default() -> Self {
        Self {
            id: use_unique_id(),
            title: String::default(),
            description: None,
            duration_in_ms: 6_000,
            is_closable: true,
            color: Color::default(),
            animation: Animation::default(),
            state: ToastState::Opening,
        }
    }
}

impl Toast {
    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, description: Element) -> Self {
        self.description = description;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = animation;
        self
    }

    pub fn duration_in_ms(mut self, duration: u32) -> Self {
        self.duration_in_ms = duration;
        self
    }

    pub fn is_closable(mut self, is_closable: bool) -> Self {
        self.is_closable = is_closable;
        self
    }
}

/// Define the state of an individual toast, used to animate the Toast
#[derive(Clone, Debug, PartialEq)]
enum ToastState {
    Opening,
    Open,
    Closing,
    // Close is not needed since it means the Toast does not exist anymore
}

impl std::fmt::Display for ToastState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToastState::Opening => "opening",
                ToastState::Open => "open",
                ToastState::Closing => "closing",
            }
        )
    }
}

/// Used to render the Toast, also update the ToasterState
#[component]
fn ToastRenderer(mut state: Signal<ToasterState>, toast: MappedSignal<Toast>) -> Element {
    let class = toast.read().build_class();

    let mut toast_state = use_signal(|| ToastState::Opening);

    let duration_in_ms = toast.read().duration_in_ms;
    let toast_animation = toast.read().animation;

    // This is to animate the Toast in and out
    use_future(move || async move {
        if toast_animation != Animation::None {
            TimeoutFuture::new(10).await;
            toast_state.set(ToastState::Open);

            let animation_play_time = 150;
            TimeoutFuture::new(duration_in_ms - animation_play_time).await;

            toast_state.set(ToastState::Closing);
            TimeoutFuture::new(animation_play_time).await;
        } else {
            TimeoutFuture::new(duration_in_ms).await;
        }

        state.write().toasts.clear();
    });

    rsx!(
        li {
            class,
            id: "{toast.read().id}",
            "data-state": toast_state.read().to_string(),
            h6 { class: "h6", "{toast.read().title}" }
            if toast.read().is_closable {
                ToastClose { state, toast_state }
            }
            {&toast.read().description}
        }
    )
}

/// Used to add a cross mark to manually close the Toast
/// The Timeout is there to let the animation some time to play
#[component]
fn ToastClose(mut state: Signal<ToasterState>, mut toast_state: Signal<ToastState>) -> Element {
    rsx!(
        button {
            class: "absolute top-4 right-4 rounded-global-radius hidden group-hover:block transition-colors focus:outline-none focus:ring focus:ring-foreground",
            r#type: "button",
            onclick: move |_| {
                spawn(async move {
                    toast_state.set(ToastState::Closing);
                    TimeoutFuture::new(150).await;
                    state.write().toasts.clear();
                });
            },
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 256 256",
                width: "15",
                height: "15",
                class: "fill-foreground/60 hover:fill-foreground",
                path { d: "M202.82861,197.17188a3.99991,3.99991,0,1,1-5.65722,5.65624L128,133.65723,58.82861,202.82812a3.99991,3.99991,0,0,1-5.65722-5.65624L122.343,128,53.17139,58.82812a3.99991,3.99991,0,0,1,5.65722-5.65624L128,122.34277l69.17139-69.17089a3.99991,3.99991,0,0,1,5.65722,5.65624L133.657,128Z" }
            }
        }
    )
}

/// Hook that returns a Fn which take a Toast as argument.
/// Use this Fn to spawn the Toast
pub fn use_toast() -> Signal<impl Fn(Toast)> {
    // Will panic if no Toaster {} upper in the DOM
    let state = use_context::<Signal<ToasterState>>();

    use_signal(|| {
        move |toast: Toast| {
            let mut state = state.clone();

            // Only allow 1 toast at a time
            state.write().toasts.clear();

            spawn(async move {
                // To let the browser refresh the UI before spawning a new Toast
                TimeoutFuture::new(100).await;
                state.write().toasts.push(toast);
            });
        }
    })
}
