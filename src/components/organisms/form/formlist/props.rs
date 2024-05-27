use crate::{atoms::{buttongroup::ButtonGroup, Separator}, components::atoms::Button};
use dioxus::prelude::*;
use props_component_macro::props_component;
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
    let class = tw_merge!(props.base(), &props.class);

    use_context_provider(|| Signal::new(FormListState::new(props.max_size)));

    rsx!(
        div { class, {props.children} }
    )
}

#[props_component(class, children)]
pub fn FormListTitle() -> Element {
    let class = tw_merge!(props.base(), &props.class);

    rsx!(
        div { class, {props.children} }
    )
}

#[props_component(class, children)]
pub fn FormListTrigger(#[props(default = false)] plus: bool) -> Element {
    // Useless div to wrap around the trigger
    let class = "inline-block";

    let mut state = use_context::<Signal<FormListState>>();

    let onclick = move |_| {
        if props.plus {
            state.write().render_one_more();
        } else {
            state.write().render_one_less();
        }
    };

    if let Some(children) = props.children {
        rsx!(
            div { onclick, class, {children} }
        )
    } else {
        let button_class = tw_merge!(props.base(), &props.class);

        let text = if props.plus { "+" } else { "-" };

        rsx!(
            Button { class: button_class, onclick, r#type: "button", {text} }
        )
    }
}

#[props_component(class)]
pub fn FormListContent(#[props(default)] list_fields: Vec<Element>) -> Element {
    let class = tw_merge!(props.base(), &props.class);

    let state = consume_context::<Signal<FormListState>>();

    let rendered_list_fields = props
        .list_fields
        .iter()
        .take(state.read().num_to_render())
        .map(|x| rsx!(
            { x }.clone(),
            Separator { class: "last:opacity-0 last:mt-0" }
        ));

    rsx!(
        div { class, { rendered_list_fields } }
    )
}
