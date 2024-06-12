use crate::atoms::{
        buttongroup::ButtonGroup,
        Separator,
    };
use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
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

#[props_component(class, children)]
pub fn FormList(#[props(default = 1)] max_size: usize) -> Element {
    use_context_provider(|| Signal::new(FormListState::new(props.max_size)));

    rsx!(
        div { class: props.class, {props.children} }
    )
}

#[props_component(class, children)]
pub fn FormListTitle() -> Element {
    rsx!(
        div { class: props.class, {props.children} }
    )
}

#[props_component(class, children)]
pub fn FormListTrigger(#[props(default = false)] plus: bool) -> Element {
    let mut state = use_context::<Signal<FormListState>>();
    
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

#[props_component(class)]
pub fn FormListContent(#[props(default)] list_fields: Vec<Element>) -> Element {
    let state = consume_context::<Signal<FormListState>>();

    let rendered_list_fields = props
        .list_fields
        .iter()
        .take(state.read().num_to_render())
        .map(|x| {
            rsx!(
                {{ x }.clone()},
                Separator { class: "last:opacity-0 last:mt-0" }
            )
        });

    rsx!(
        div { class: props.class, { rendered_list_fields } }
    )
}