use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

struct AccordionState(bool);
struct AccordionId(String);

#[derive(PartialEq, Props, Clone, Component)]
pub struct AccordionProps {
    #[props(default = crate::hooks::use_unique_id())]
    id: String,

    #[props(default = false)]
    multi_open: bool,

    children: Element,

    // Styling
    #[props(default)]
    class: String,
}

// TODO Add multi_open functionality
impl Component for AccordionProps {
    fn view(self) -> Element {
        use_context_provider(|| Signal::new(AccordionId(self.id.clone())));

        rsx!({ self.children })
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct AccordionItemProps {
    #[props(default = crate::hooks::use_unique_id())]
    id: String,

    children: Element,

    #[props(default)]
    class: String,
}

impl Component for AccordionItemProps {
    fn view(self) -> Element {
        let class = super::style::AccordionItemClass::builder().with_class("");

        use_context_provider(|| Signal::new(AccordionState(false)));

        rsx!(
            div { class: "{class}", id: self.id, {self.children} }
        )
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct AccordionTriggerProps {
    #[props(default = crate::hooks::use_unique_id())]
    id: String,

    children: Element,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for AccordionTriggerProps {
    fn view(self) -> Element {
        let button_closure = move |_: Event<MouseData>| {
            if read_accordion_state() {
                use_accordion_state().set(AccordionState(false));
            } else {
                use_accordion_state().set(AccordionState(true));
            }
        };

        let class = super::style::AccordionTriggerClass::builder().with_class(self.class);

        rsx!( button { class: "{class}", id: self.id, onclick: button_closure, "Button" } )
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct AccordionContentProps {
    #[props(default = crate::hooks::use_unique_id())]
    id: String,

    children: Element,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for AccordionContentProps {
    fn view(self) -> Element {
        let mut elem_height_sig = use_signal(|| "".to_string());

        let final_height = match read_accordion_state() {
            true => elem_height_sig(),
            false => "0".to_string(),
        };

        // Use a signal to pass it through the closure
        let sig = use_signal(|| self.id.clone());

        // TODO Change unwraps
        let onmounted = move |_| async move {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let element_height = document.get_element_by_id(&sig()).unwrap().scroll_height();
            elem_height_sig.set(format!("{}px", element_height));
        };

        let class = super::style::AccordionContentClass::builder().with_class(self.class);

        rsx!(
            div {
                id: "{self.id}",
                class: "{class}",
                height: final_height,
                onmounted: onmounted,
                {self.children}
            }
        )
    }
}

fn use_accordion_state() -> Signal<AccordionState> {
    use_context::<Signal<AccordionState>>()
}

fn read_accordion_state() -> bool {
    use_context::<Signal<AccordionState>>().read().0
}

// #[derive(PartialEq, Props, Clone, Component)]
// pub struct AccordionProps {
//     #[props(extends = GlobalAttributes)]
//     attributes: Vec<Attribute>,

//     children: Element,

//     // Styling
//     #[props(default)]
//     class: String,
// }

// impl Component for AccordionProps {
//     fn view(mut self) -> Element {
//         let class = AccordionClass::builder().with_class(self.class);

//         // TODO How can add we extend GlobalAttributes so we don't have to do this in every accordion component
//         self.attributes
//             .push(Attribute::new("data-open", "close", None, true));

//         rsx!(
//             div { ..self.attributes, class: "{class}", {self.children} }
//         )
//     }
// }

// struct AccordionState(bool);

// #[derive(PartialEq, Props, Clone, Component)]
// pub struct AccordionItemProps {
//     #[props(extends = GlobalAttributes)]
//     attributes: Vec<Attribute>,

//     children: Element,

//     // Styling
//     #[props(default)]
//     class: String,
// }

// impl Component for AccordionItemProps {
//     fn view(mut self) -> Element {
//         use_context_provider(|| Signal::new(AccordionState(false)));

//         let class = AccordionItemClass::builder().with_class(self.class);

//         self.attributes
//             .push(Attribute::new("data-open", "close", None, true));

//         rsx!(
//             div { ..self.attributes, class: "{class}", { self.children } }
//         )
//     }
// }

// #[derive(PartialEq, Props, Clone, Component)]
// pub struct AccordionTriggerProps {
//     #[props(extends = GlobalAttributes)]
//     attributes: Vec<Attribute>,

//     children: Element,

//     // Styling
//     #[props(default)]
//     class: String,
// }

// impl Component for AccordionTriggerProps {
//     fn view(mut self) -> Element {
//         log::debug!("Rendering AccordionTrigger");
//         let class = AccordionTriggerClass::builder().with_class(self.class);

//         self.attributes
//             .push(Attribute::new("data-open", "close", None, true));

//         let trigger_closure = move |_: Event<MouseData>| {
//             toggle_accordion(use_context::<Signal<AccordionState>>());
//         };

//         let accordion_state = use_context::<Signal<AccordionState>>();
//         let state = accordion_state.read().0;
//         if state {
//             self.attributes.last_mut().unwrap().value = AttributeValue::Text("open".to_string());
//         } else {
//             self.attributes.last_mut().unwrap().value = AttributeValue::Text("close".to_string());
//         }

//         rsx!(
//             button { ..self.attributes, class: "{class}", onclick:trigger_closure, { self.children } span{ class:"transition-transform duration-200", "A"} }
//         )
//     }
// }

// #[derive(PartialEq, Props, Clone, Component)]
// pub struct AccordionContentProps {
//     #[props(extends = GlobalAttributes)]
//     attributes: Vec<Attribute>,

//     children: Element,

//     // Styling
//     #[props(default)]
//     class: String,
// }

// impl Component for AccordionContentProps {
//     fn view(mut self) -> Element {
//         log::debug!("Rendering AccordionContent");

//         let class = AccordionContentClass::builder().with_class(self.class);

//         self.attributes
//             .push(Attribute::new("data-state", "close", None, true));

//         let accordion_state = use_context::<Signal<AccordionState>>();
//         let state = accordion_state.read().0;
//         let mut hidden = true;
//         if state {
//             self.attributes.last_mut().unwrap().value = AttributeValue::Text("open".to_string());
//             hidden = false;
//         } else {
//             self.attributes.last_mut().unwrap().value = AttributeValue::Text("close".to_string());
//             hidden = true;
//         }

//         rsx!(
//             div { ..self.attributes, class: "{class}", hidden:false, { self.children } }
//         )
//     }
// }

// fn toggle_accordion(mut accordion_context: Signal<AccordionState>) {
//     if accordion_context.read().0 {
//         accordion_context.set(AccordionState(false));
//     } else {
//         accordion_context.set(AccordionState(true));
//     }
// }
