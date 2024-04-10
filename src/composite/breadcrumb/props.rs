use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props!(BreadcrumbProps {});

impl Component for BreadcrumbProps {
    fn view(self) -> Element {
        let class = super::BreadcrumbClass::builder().with_class(self.class);

        rsx!(
            ol { class: "{class}", id: self.id, { self.children } }
        )
    }
}

props!(BreadcrumbItemProps {});

impl Component for BreadcrumbItemProps {
    fn view(self) -> Element {
        let class = super::BreadcrumbItemClass::builder().with_class(self.class);

        rsx!(
            li { class: "{class}", id: self.id, { self.children } }
        )
    }
}

props!(BreadcrumbSeparatorProps {});

impl Component for BreadcrumbSeparatorProps {
    fn view(self) -> Element {
        let class = super::BreadcrumbSeparatorClass::builder().with_class(self.class);

        rsx!(
            li { class: "{class}", aria_hidden: "true", id: self.id,
                if self.children == None {
                    "\u{203A}"
                } else {
                    { self.children }
                }
            }
        )
    }
}
