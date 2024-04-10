use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props_no_children!(InputProps {
    #[props(extends = input)]
    attribute: Vec<Attribute>,

    #[props(optional)]
    oninput: Option<EventHandler<FormEvent>>,

    #[props(default)]
    size: InputSize,
});

impl Component for InputProps {
    fn view(self) -> Element {
        let class = InputClass::builder().size(self.size).with_class(self.class);

        let oninput = move |event| {
            if let Some(oc) = &self.oninput {
                oc.call(event)
            }
        };

        rsx! { input { ..self.attribute, oninput: oninput, class: class } }
    }
}
