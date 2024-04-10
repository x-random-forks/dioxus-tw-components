use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props_no_children!(TextAreaProps {
    #[props(extends = textarea)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    oninput: Option<EventHandler<FormEvent>>,

});

impl Component for TextAreaProps {
    fn view(self) -> Element {
        let class = TextAreaClass::builder().with_class(self.class);

        let oninput = move |event| {
            if let Some(oc) = &self.oninput {
                oc.call(event)
            }
        };

        // TODO add a default placeholder

        rsx!( textarea { ..self.attributes, class: "{class}", oninput: oninput } )
    }
}
