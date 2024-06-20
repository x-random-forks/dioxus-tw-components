use crate::{attributes::*, hooks::use_unique_id};
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

const TOAST_ID: &str = "dx-toast";

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct ToasterProps {
    #[props(extends = ol, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// The toaster must wrap around your App/as high as possible in your app to be used
pub fn Toaster(mut props: ToasterProps) -> Element {
    props.update_class_attribute();

    let mut state =
        use_context_provider::<Signal<ToasterState>>(|| Signal::new(ToasterState::default()));

    let _ = use_effect(move || {
        log::debug!("Runnning Effect");

        state.read();

    });
    
    let _ = use_resource(move || async move {
        log::debug!("Runnning Effect");
        
        if let Some(last) = state.write().toasts.last_mut() {
            log::debug!("{:#?}", last);
        }

    });
    // let binding = state.read();
    // let Some(last_toast) = binding.toasts.last() else {
    //         return;
    // };
    // // Using an eval to get a working timeout... Did not find a reliable crate to do it yet
    // // Waiting for the eval to send back a message after the timeout is done to remove the Toast from the vec
    // // Little problem is that if the user change page it won't get to receive the message back..
    // let mut eval = eval(
    //     r#"
    //     let toast_duration = await dioxus.recv();

    //     setTimeout(() => {
    //         dioxus.send();
    //     }, toast_duration)
    //     "#,
    // );
    // // Send the duration to the eval
    // let _ = eval.send(last_toast.duration_in_ms.into());

    // // Wait for the timeout to be over
    // let _ = eval.recv().await;

    // // Delete the Toast from Vec<Toast>
    // binding
    //     .toasts
    //     .retain(|toast_from_vec| toast_from_vec.id != last_toast.id);

    // spawn(async move {
    //     let mut binding = state.write();
    //     let Some(last_toast) = binding.toasts.last() else {
    //         return;
    //     };
    //     // Using an eval to get a working timeout... Did not find a reliable crate to do it yet
    //     // Waiting for the eval to send back a message after the timeout is done to remove the Toast from the vec
    //     // Little problem is that if the user change page it won't get to receive the message back..
    //     let mut eval = eval(
    //         r#"
    //         let toast_duration = await dioxus.recv();

    //         setTimeout(() => {
    //             dioxus.send();
    //         }, toast_duration)
    //         "#,
    //     );
    //     // Send the duration to the eval
    //     let _ = eval.send(last_toast.duration_in_ms.into());

    //     // Wait for the timeout to be over
    //     let _ = eval.recv().await;

    //     // // Delete the Toast from Vec<Toast>
    //     // binding
    //     //     .toasts
    //     //     .retain(|toast_from_vec| toast_from_vec.id != last_toast.id);
    // });

    rsx!(
        {props.children},
        ol { role: "alert", id: TOAST_ID, ..props.attributes,
            for toast in state.read().toasts.iter() {
                RenderToast { toast: toast.clone() }
            }
        }
    )
}

#[derive(Clone, Debug, PartialEq, UiComp)]
pub struct Toast {
    id: String,
    title: String,
    description: Element,
    pub color: Color,
    duration_in_ms: u32,
}

impl Default for Toast {
    fn default() -> Self {
        Self {
            id: use_unique_id(),
            title: String::new(),
            description: None,
            duration_in_ms: 50000,
            color: Color::default(),
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

    pub fn duration_in_ms(mut self, duration: u32) -> Self {
        self.duration_in_ms = duration;
        self
    }
}

#[component]
pub fn RenderToast(toast: Toast) -> Element {
    let class = toast.build_class();

    let mut state = use_context::<Signal<ToasterState>>();

    rsx!(
        li { class, id: "{toast.id}",
            h6 { class: "h6", "{toast.title}" }
            button {
                class: "absolute top-4 right-4 hidden group-hover:block transition-all",
                r#type: "button",
                onclick: move |_| {
                    state.write().toasts.retain(|toast_from_vec| toast_from_vec.id != toast.id);
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
            {toast.description}
        }
    )
}

pub fn use_toast() -> impl Fn(Toast) {
    // Will panic if no Toaster {} upper in the DOM
    let state = use_context::<Signal<ToasterState>>();

    move |toast: Toast| {
        let mut state = state.clone();

        let id = toast.id.clone();
        let toast_duration = toast.duration_in_ms;

        state.write().toasts.push(toast);

        // spawn(async move {
        //     // Using an eval to get a working timeout... Did not find a reliable crate to do it yet
        //     // Waiting for the eval to send back a message after the timeout is done to remove the Toast from the vec
        //     // Little problem is that if the user change page it won't get to receive the message back..
        //     let mut eval = eval(
        //         r#"
        //             let toast_duration = await dioxus.recv();

        //             setTimeout(() => {
        //                 dioxus.send();
        //             }, toast_duration)
        //         "#,
        //     );
        //     // Send the duration to the eval
        //     let _ = eval.send(toast_duration.into());

        //     // Wait for the timeout to be over
        //     let _ = eval.recv().await;

        //     // Delete the Toast from Vec<Toast>
        //     state.write().toasts.retain(|toast| toast.id != id);
        // });
    }
}

/// Used to keep track of the number of toasts created
/// This number is used to create a unique ID in the DOM to then grab it and do whatever with it
/// In this case we use it to remove it after the toast's timeout is up
/// This state also contains the current position of the Toast {} since all
/// the toasts are by default just <li> tag wrapped by the Toast component which is just an <ol> tag
pub struct ToasterState {
    max_toast: u32,
    toast_count: u32,
    toasts: Vec<Toast>,
}

impl std::default::Default for ToasterState {
    fn default() -> Self {
        ToasterState {
            max_toast: 3,
            toast_count: 0,
            toasts: vec![],
        }
    }
}

impl ToasterState {
    pub fn new(max_toast: u32) -> Self {
        ToasterState {
            max_toast,
            toast_count: 0,
            toasts: vec![],
        }
    }
    // fn add_new_toast(&mut self, toast: Toast) {
    //     let id = toast.id.clone();
    //     let toast_duration = toast.duration_in_ms;

    //     self.toasts.push(toast);

    //     spawn(async move {
    //         // Using a eval to retrieve the toast id, and setting a timeout to remove the toast when its duration is up
    //         let eval = eval(
    //             r#"
    //                 let toast_id = await dioxus.recv();

    //                 let toast = document.getElementById(toast_id);
    //                 if (toast == undefined) {
    //                     return;
    //                 }

    //                 let toast_duration = await dioxus.recv();

    //                 setTimeout(() => {
    //                     toast.remove();
    //                 }, toast_duration)
    //             "#,
    //         );

    //         let _ = eval.send(id.into());
    //         let _ = eval.send(toast_duration.into());
    //     });
    // }

    // // Old
    // fn increment_toast_count(&mut self) {
    //     self.toast_count += 1;
    // }

    // fn decrement_toast_count(&mut self) {
    //     self.toast_count -= 1;
    // }

    // fn get_current_position(&self) -> ToastPosition {
    //     self.current_position
    // }

    // fn current_position(&mut self, position: ToastPosition) {
    //     self.current_position = position;
    // }

    // fn build_toast_id(&self) -> String {
    //     format!("{}-li-{}", TOAST_ID, self.toast_count)
    // }
}

// #[derive(Default, Clone, PartialEq, Copy)]
// pub enum ToastPosition {
//     TopLeft,
//     TopRight,
//     BottomLeft,
//     #[default]
//     BottomRight,
// }

// impl FromStr for ToastPosition {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "bottomleft" => Ok(ToastPosition::BottomLeft),
//             "topleft" => Ok(ToastPosition::TopLeft),
//             "topright" => Ok(ToastPosition::TopRight),
//             "bottomright" | _ => Ok(ToastPosition::BottomRight),
//         }
//     }
// }

// impl ToastPosition {
//     /// Will always return a position, if spelling mistake will return ToastPosition::default()
//     pub fn str_to_toast_pos<T: ToString>(str: T) -> ToastPosition {
//         str.to_string().parse().unwrap()
//     }
// }

// /// Changes the position of all the toasts currently active
// /// Uses ToastState, if not declared will do nothing
// pub fn use_toast_set_position(toast_position: ToastPosition) {
//     // TODO use try_consume_context() instead
//     let mut state = use_context::<Signal<ToasterState>>();

//     if let Ok(element) = use_element_by_id(TOAST_ID) {
//         let current_position = state.read().get_current_position();
//         if toast_position != current_position {
//             // Change <ol> tag class to fit its new position

//             let mut class = element.get_attribute("class").unwrap_or_default();

//             match class.find(&current_position.to_string()) {
//                 None => class.push_str(&toast_position.to_string()),
//                 Some(index) => class.replace_range(
//                     index..index + &current_position.to_string().len(),
//                     &toast_position.to_string(),
//                 ),
//             };

//             state.write().current_position(toast_position);

//             element
//                 .set_attribute("class", &class)
//                 .unwrap_or_else(|e| log::error!("Failed setting new toast class {:#?}", e));
//         }
//     }
// }

// /// Return a string with the id of the newly created toast in the DOM if success, else a web_sys::JsValue Err
// /// Will also panic if the function can't find the signal (use_context())
// pub fn use_toast(toast: Toast) -> Result<String, JsValue> {
//     // Try getting the Toast in the DOM
//     let element = use_element_by_id(TOAST_ID)?;

//     // This can panic
//     let mut state = use_context::<Signal<ToastState>>();

//     let new_toast_id = state.read().build_toast_id();

//     let html = toast.build(&new_toast_id);

//     // Insert the newly build toast in the dom
//     element.insert_adjacent_html("afterbegin", &html)?;

//     // If succeeded to insert it increment toast_count
//     state.write().increment_toast_count();

//     // Start the toast timeout if there is any
//     if let Some(timeout) = toast.timeout {
//         begin_toast_timeout(&new_toast_id, timeout, state)?;
//     }

//     Ok(new_toast_id)
// }

// /// Return the id of the timeout if success
// /// else a Err(web_sys::JsValue)
// fn begin_toast_timeout(
//     toast_id: &str,
//     timeout: i32,
//     mut state: Signal<ToastState>,
// ) -> Result<i32, JsValue> {
//     let window = use_window()?;

//     let element = use_element_by_id(toast_id).unwrap();

//     let closure = Closure::wrap(Box::new(move || {
//         element.remove();
//         state.write().decrement_toast_count();
//     }) as Box<dyn FnMut()>);

//     let timeout_result = use_set_timeout(&window, &closure, timeout);

//     closure.forget();

//     timeout_result
// }
