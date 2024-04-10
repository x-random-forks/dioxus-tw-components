use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props_no_children!(FormRangeProps {
    #[props(extends = input)]
    attributes: Vec<Attribute>,

    // TODO implement <datalist>
    // #[props(default = "".to_string())]
    // list: String,
    #[props(optional)]
    oninput: Option<EventHandler<FormEvent>>,
});

impl Component for FormRangeProps {
    fn view(self) -> Element {
        let class = super::FormRangeClass::builder().with_class(self.class);

        let oninput = move |event| {
            if let Some(oc) = &self.oninput {
                oc.call(event)
            }
        };

        rsx!(
            input {
                ..self.attributes,
                r#type: "range",
                // list: "{self.list}",
                class: "{class}",
                oninput: oninput
            }
        )
    }
}
