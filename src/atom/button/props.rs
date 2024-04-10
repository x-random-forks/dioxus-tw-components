use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props! {
    ButtonProps {
        #[props(extends = button)]
        attributes: Vec<Attribute>,

        #[props(optional)]
        onclick: Option<EventHandler<MouseEvent>>,

        #[props(default)]
        variant: super::ButtonVariant,
        #[props(default)]
        size: super::ButtonSize,
    }
}

impl Component for ButtonProps {
    fn view(self) -> Element {
        let class = super::ButtonClass::builder()
            .variant(self.variant)
            .size(self.size)
            .with_class(self.class);

        let onclick = move |event| {
            if let Some(oc) = &self.onclick {
                oc.call(event)
            }
        };

        rsx!(
            button {
                ..self.attributes,
                class: "{class}",
                id: self.id,
                onclick: onclick,
                {self.children}
            }
        )
    }
}
