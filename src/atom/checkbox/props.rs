use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props!(CheckboxProps {
    #[props(extends = input)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    oninput: Option<EventHandler<FormEvent>>,

    #[props(default)]
    color: CheckboxColor,
});

// REVIEW: Should split into several components and move to composite ?
// Reasoning is the label encapsulate both the input and the children (usually some text)
impl Component for CheckboxProps {
    fn view(self) -> Element {
        let class = CheckboxClass::builder()
            .color(self.color)
            .with_class(self.class);

        let oninput = move |event| {
            if let Some(oc) = &self.oninput {
                oc.call(event)
            }
        };

        rsx!(
            label { class: "cursor-pointer gap-x-1 flex items-center",
                input {
                    ..self.attributes,
                    id: self.id,
                    r#type: "checkbox",
                    class: "{class}",
                    oninput: oninput
                }
                div { class: "peer-disabled:opacity-30", {self.children} }
            }
        )
    }
}
