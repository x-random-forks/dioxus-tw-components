use crate::atoms::{buttongroup::ButtonGroup, Separator};
use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

pub struct FormListState {
    max_size: usize,
    current_size: usize,
}

impl FormListState {
    fn new(max_size: usize) -> Self {
        FormListState {
            max_size,
            current_size: 1,
        }
    }

    fn num_to_render(&self) -> usize {
        self.current_size
    }

    fn render_one_more(&mut self) {
        if self.current_size < self.max_size {
            self.current_size += 1;
        }
    }

    fn render_one_less(&mut self) {
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
    use_context_provider(|| Signal::new(FormListState::new(props.max_size)));

    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct FormListTitleProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn FormListTitle(mut props: FormListTitleProps) -> Element {
    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct FormListTriggerProps {
    #[props(default = false)]
    plus: bool,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

// TODO refactor
pub fn FormListTrigger(mut props: FormListTriggerProps) -> Element {
    let mut state = use_context::<Signal<FormListState>>();

    props.update_class_attribute();

    rsx!(
        ButtonGroup { class: "flex flex-col divide-none bg-foreground/5",
            button {
                r#type: "button",
                class: "px-2 py-1.5 border-b border-border font-bold text-foreground rounded-t-global-radius hover:bg-foreground/20 active:bg-foreground-30",
                onclick: move |_| {
                    state.write().render_one_more();
                },
                "+"
            }
            button {
                r#type: "button",
                class: "px-2 py-1.5 font-bold text-foreground rounded-b-global-radius first:rounded-b-none hover:bg-foreground/20 active:bg-foreground-30",
                onclick: move |_| {
                    state.write().render_one_less();
                },
                "-"
            }
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct FormListContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default)]
    list_fields: Vec<Element>,
}

pub fn FormListContent(mut props: FormListContentProps) -> Element {
    let state = use_context::<Signal<FormListState>>();

    props.update_class_attribute();

    let rendered_list_fields = props
        .list_fields
        .iter()
        .take(state.read().num_to_render())
        .map(|x| {
            rsx!(
                { { x }.clone() },
                Separator { class: "last:opacity-0 last:mt-0" }
            )
        });

    rsx!(
        div { ..props.attributes, { rendered_list_fields } }
    )
}
