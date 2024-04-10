use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props!(TableProps {
    #[props(extends = table)]
    attributes: Vec<Attribute>,
});

impl Component for TableProps {
    fn view(self) -> Element {
        let class = super::style::TableClass::builder().with_class(self.class);

        rsx!(
            table { ..self.attributes, class: class, { self.children } }
        )
    }
}

props!(TableHeaderProps {
    #[props(extends = thead)]
    attributes: Vec<Attribute>,
});

impl Component for TableHeaderProps {
    fn view(self) -> Element {
        let class = super::style::TableHeaderClass::builder().with_class(self.class);

        rsx!(
            thead { ..self.attributes, class: class, { self.children } }
        )
    }
}

props!(TableBodyProps {
    #[props(extends = tbody)]
    attributes: Vec<Attribute>,
});

impl Component for TableBodyProps {
    fn view(self) -> Element {
        let class = super::style::TableBodyClass::builder().with_class(self.class);

        rsx!(
            tbody { ..self.attributes, class: class, { self.children } }
        )
    }
}

props!(TableFooterProps {
    #[props(extends = tfoot)]
    attributes: Vec<Attribute>,
});

impl Component for TableFooterProps {
    fn view(self) -> Element {
        let class = super::style::TableFooterClass::builder().with_class(self.class);

        rsx!(
            tfoot { ..self.attributes, class: class, { self.children } }
        )
    }
}

props!(TableHeadProps {
    #[props(extends = th)]
    attributes: Vec<Attribute>,
});

impl Component for TableHeadProps {
    fn view(self) -> Element {
        let class = super::style::TableHeadClass::builder().with_class(self.class);

        rsx!(
            th { ..self.attributes, class: class, { self.children } }
        )
    }
}

props!(TableRowProps {
    #[props(extends = tr)]
    attributes: Vec<Attribute>,
});

impl Component for TableRowProps {
    fn view(self) -> Element {
        let class = super::style::TableRowClass::builder().with_class(self.class);

        rsx!(
            tr { ..self.attributes, class: class, { self.children } }
        )
    }
}

props!(TableCellProps {
    #[props(extends = td)]
    attributes: Vec<Attribute>,
});

impl Component for TableCellProps {
    fn view(self) -> Element {
        let class = super::style::TableCellClass::builder().with_class(self.class);

        rsx!(
            td { ..self.attributes, class: class, { self.children } }
        )
    }
}

props!(TableCaptionProps {
    #[props(extends = caption)]
    attributes: Vec<Attribute>,
});

impl Component for TableCaptionProps {
    fn view(self) -> Element {
        let class = super::style::TableCaptionClass::builder().with_class(self.class);

        rsx!(
            caption { ..self.attributes, class: class, { self.children } }
        )
    }
}
