use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

pub struct FormListState {
    max_size: usize,
    current_size: usize,
}

impl FormListState {
    fn new() -> Self {
        FormListState {
            max_size: 1,
            current_size: 1,
        }
    }

    fn get_max_size(&self) -> usize {
        self.max_size
    }

    fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
    }

    fn get_current_size(&self) -> usize {
        self.current_size
    }

    fn add_one(&mut self) {
        if self.current_size < self.max_size {
            self.current_size += 1;
        }
    }

    fn remove_one(&mut self) {
        if self.current_size > 1 {
            self.current_size -= 1;
        }
    }
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct FormListProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = 1)]
    max_size: usize,

    children: Element,
}

pub fn FormList(mut props: FormListProps) -> Element {
    use_context_provider(|| Signal::new(FormListState::new()));

    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct FormListTriggerPlusProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn FormListTriggerPlus(mut props: FormListTriggerPlusProps) -> Element {
    let mut state = use_context::<Signal<FormListState>>();

    props.update_class_attribute();

    rsx!(
        div {
            ..props.attributes,
            onclick: move |_| {
                state.write().add_one();
            },
            {props.children}
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct FormListTriggerMinusProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn FormListTriggerMinus(mut props: FormListTriggerMinusProps) -> Element {
    let mut state = use_context::<Signal<FormListState>>();

    props.update_class_attribute();

    rsx!(
        div {
            ..props.attributes,
            onclick: move |_| {
                state.write().remove_one();
            },
            {props.children}
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct FormListContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default)]
    list_fields: Vec<Element>,

    #[props(default, optional)]
    pub animation: ReadOnlySignal<Animation>,
}

pub fn FormListContent(mut props: FormListContentProps) -> Element {
    let mut state = use_context::<Signal<FormListState>>();

    props.update_class_attribute();

    let max_size = props.list_fields.len();
    use_effect(move || {
        state.write().set_max_size(max_size);
    });

    // Height all the fields together
    let mut fields_height = use_signal(|| 0.);

    // Use the ounmonted event to retrieve the height of the fields
    let onmounted = move |event: MountedEvent| async move {
        let rect = event.get_client_rect().await;
        if let Ok(rect) = rect {
            fields_height.set(rect.size.height);
        }
    };

    let height = use_memo(move || {
        // If somehow the height of the fields is 0, return an empty string, to still have the div showing in entirety
        if fields_height() == 0. {
            return "".to_string();
        }
        // Set the height of the div to be the height of a single field times the current size to render
        format!(
            "height: {}px;",
            state.read().get_current_size()
                * (fields_height() as usize / state.read().get_max_size())
        )
    });

    rsx!(
        div { ..props.attributes, onmounted, style: height,
            for field in props.list_fields.iter() {
                { field.clone() }
            }
        }
    )
}
