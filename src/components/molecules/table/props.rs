use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TableProps {
    #[props(extends = table, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn Table(mut props: TableProps) -> Element {
    props.build_class();

    rsx!(
        table { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TableHeaderProps {
    #[props(extends = thead, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn TableHeader(mut props: TableHeaderProps) -> Element {
    props.build_class();

    rsx!(
        thead { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TableBodyProps {
    #[props(extends = tbody, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn TableBody(mut props: TableBodyProps) -> Element {
    props.build_class();

    rsx!(
        tbody { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TableFooterProps {
    #[props(extends = tfoot, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn TableFooter(mut props: TableFooterProps) -> Element {
    props.build_class();

    rsx!(
        tfoot { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TableHeadProps {
    #[props(extends = th, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

pub fn TableHead(mut props: TableHeadProps) -> Element {
    props.build_class();

    let onclick = move |event| props.onclick.call(event);

    rsx!(
        th { ..props.attributes, onclick, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TableRowProps {
    #[props(extends = tr, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn TableRow(mut props: TableRowProps) -> Element {
    props.build_class();

    rsx!(
        tr { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TableCellProps {
    #[props(extends = td, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn TableCell(mut props: TableCellProps) -> Element {
    props.build_class();

    rsx!(
        td { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TableCaptionProps {
    #[props(extends = caption, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn TableCaption(mut props: TableCaptionProps) -> Element {
    props.build_class();

    rsx!(
        caption { ..props.attributes, {props.children} }
    )
}
