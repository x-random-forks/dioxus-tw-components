use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props_no_children!(SeparatorProps {
    #[props(default = false)]
    vertical: bool,
});

impl Component for SeparatorProps {
    fn view(self) -> Element {
        let class = super::SeparatorClass::builder()
            .vertical(self.vertical.into())
            .with_class(self.class);

        rsx!(div { class: "{class}" })
    }
}
