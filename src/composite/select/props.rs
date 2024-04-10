use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props!(SelectGroupProps {
    #[props(extends = select)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    oninput: Option<EventHandler<FormEvent>>,
});

impl Component for SelectGroupProps {
    fn view(self) -> Element {
        let class = SelectGroupClass::builder().with_class(self.class);

        let oninput = move |event| {
            if let Some(oc) = &self.oninput {
                oc.call(event)
            }
        };

        rsx!(
            select {
                ..self.attributes,
                class: "{class}",
                id: self.id,
                oninput: oninput,
                {self.children}
            }
        )
    }
}

// What will be shown by default in the SelectGroup when nothing is selected yet
props!(SelectPlaceholderProps {});

impl Component for SelectPlaceholderProps {
    fn view(self) -> Element {
        let class = SelectPlaceholderClass::builder().with_class(self.class);

        rsx!(
            option {
                disabled: true,
                selected: true,
                value: "",
                class: "{class}",
                id: self.id,
                {self.children}
            }
        )
    }
}

// SelectLabel is used to group SelectItems together under a common label
props!(SelectLabelProps {
    #[props(extends = optgroup)]
    attributes: Vec<Attribute>,
});

impl Component for SelectLabelProps {
    fn view(self) -> Element {
        let class = SelectLabelClass::builder().with_class(self.class);

        rsx!(
            optgroup { ..self.attributes, class: "{class}", id: self.id, {self.children} }
        )
    }
}

props!(SelectItemProps {
    #[props(extends = option)]
    attributes: Vec<Attribute>,
});

impl Component for SelectItemProps {
    fn view(self) -> Element {
        let class = SelectItemClass::builder().with_class(self.class);

        rsx!(
            option { ..self.attributes, class: "{class}", id: self.id, {self.children} }
        )
    }
}
