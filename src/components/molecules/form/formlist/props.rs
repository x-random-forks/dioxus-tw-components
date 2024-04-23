use crate::components::atoms::button::*;
use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

// TODO : This is more of an example, probably need to use it before refining implementation
#[props_component(class, id)]
pub fn FormList(#[props(default)] group_vec: Vec<Element>) -> Element {
    let class = tw_merge!(props.class);

    let mut group_to_render = use_signal(|| 1);

    let rendered_group_vec = props
        .group_vec
        .iter()
        .take(group_to_render())
        .map(|x| rsx!(
            { x }.clone(),
            div { class: "h-4" }
        ));

    let vec_size = props.group_vec.len();

    let button_closure_plus = move |_| {
        if group_to_render() < vec_size {
            group_to_render += 1;
        }
    };

    let button_closure_minus = move |_| {
        if group_to_render() > 1 {
            group_to_render -= 1;
        }
    };

    rsx!(
        div { class: class, id: props.id,
            Button { onclick: button_closure_plus, r#type: "button", "+" }
            Button { onclick: button_closure_minus, r#type: "button", "-" }
            { rendered_group_vec }
        }
    )
}

// #[derive(PartialEq, Props, Clone, Component)]
// pub struct FormListProps {
//     #[props(default)]
//     group_vec: Vec<Element>,
// }

// impl Component for FormListProps {
//     fn view(self) -> Element {
//         let mut group_to_render = use_signal(|| 1);

//         let rendered_group_vec = self
//             .group_vec
//             .iter()
//             .take(group_to_render())
//             .map(|x| rsx!({ x }.clone(), div { class: "h-4" }));

//         let vec_size = self.group_vec.len();

//         let button_closure_plus = move |_| {
//             if group_to_render() < vec_size {
//                 group_to_render += 1;
//             }
//         };

//         let button_closure_minus = move |_| {
//             if group_to_render() > 1 {
//                 group_to_render -= 1;
//             }
//         };

//         rsx!(
//             Button { onclick: button_closure_plus, r#type: "button", "+" }
//             Button { onclick: button_closure_minus, r#type: "button", "-" }
//             { rendered_group_vec }
//         )
//     }
// }
