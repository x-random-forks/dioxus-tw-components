use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props!(DropdownProps {});

impl Component for DropdownProps {
    fn view(self) -> Element {
        let class = DropdownClass::builder().with_class(self.class);

        rsx!(
            div { class: "{class}", id: self.id, {self.children} }
        )
    }
}

props!(DropdownToggleProps {});

impl Component for DropdownToggleProps {
    fn view(self) -> Element {
        let class = DropdownToggleClass::builder().with_class(self.class);

        rsx!(
            div { class: "{class}", id: self.id, {self.children} }
        )
    }
}

// Use HTML/CSS black magic to render the dropdown content
// group-focus-within:block
props!(DropdownContentProps {});

impl Component for DropdownContentProps {
    fn view(self) -> Element {
        let class = DropdownContentClass::builder().with_class(self.class);

        rsx!(
            div { class: "{class}", id: self.id, {self.children} }
        )
    }
}
