use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props!(TableProps {});

impl Component for TableProps {
    fn view(self) -> Element {
        let class = super::style::TableClass::builder().with_class(self.class);

        rsx!(
            table { class: class, { self.children } }
        )
    }
}

props!(TableHeaderProps {});

impl Component for TableHeaderProps {
    fn view(self) -> Element {
        let class = super::style::TableHeaderClass::builder().with_class(self.class);

        rsx!(
            thead { class: class, { self.children } }
        )
    }
}

props!(TableBodyProps {});

impl Component for TableBodyProps {
    fn view(self) -> Element {
        let class = super::style::TableBodyClass::builder().with_class(self.class);

        rsx!(
            tbody { class: class, { self.children } }
        )
    }
}

props!(TableFooterProps {});

impl Component for TableFooterProps {
    fn view(self) -> Element {
        let class = super::style::TableFooterClass::builder().with_class(self.class);

        rsx!(
            tfoot { class: class, { self.children } }
        )
    }
}

// TODO Enum Scope, see link below
props!(TableHeadProps {
    // https://developer.mozilla.org/fr/docs/Web/HTML/Element/th#scope
    #[props(default)]
    scope: String,
});

impl Component for TableHeadProps {
    fn view(self) -> Element {
        let class = super::style::TableHeadClass::builder().with_class(self.class);

        rsx!(
            th { class: class, scope: self.scope, { self.children } }
        )
    }
}

props!(TableRowProps {});

impl Component for TableRowProps {
    fn view(self) -> Element {
        let class = super::style::TableRowClass::builder().with_class(self.class);

        rsx!(
            tr { class: class, { self.children } }
        )
    }
}

props!(TableCellProps {});

impl Component for TableCellProps {
    fn view(self) -> Element {
        let class = super::style::TableCellClass::builder().with_class(self.class);

        rsx!(
            td { class: class, { self.children } }
        )
    }
}

props!(TableCaptionProps {});

impl Component for TableCaptionProps {
    fn view(self) -> Element {
        let class = super::style::TableCaptionClass::builder().with_class(self.class);

        rsx!(
            caption { class: class, { self.children } }
        )
    }
}
