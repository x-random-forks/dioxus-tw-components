use crate::{atom::button::*, *};
use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct FormListProps {
    #[props(default)]
    group_vec: Vec<Element>,
}

impl Component for FormListProps {
    fn view(self) -> Element {
        let mut group_to_render = use_signal(|| 1);

        let rendered_group_vec = self
            .group_vec
            .iter()
            .take(group_to_render())
            .map(|x| rsx!(
                { x }.clone(),
                div { class: "h-4" }
            ));

        let vec_size = self.group_vec.len();
        let button_closure_plus = move |_| {
            log::debug!("Plus Button Clicked");
            if group_to_render() < vec_size {
                group_to_render += 1;
            }
        };
        let button_closure_minus = move |_| {
            log::debug!("Minus Button Clicked");
            if group_to_render() > 1 {
                group_to_render -= 1;
            }
        };

        rsx!(
            Button { r#type: "button", variant: ButtonVariant::Outline, onclick: button_closure_plus, "+" }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: button_closure_minus,
                "-"
            }
            { rendered_group_vec }
        )
    }
}
