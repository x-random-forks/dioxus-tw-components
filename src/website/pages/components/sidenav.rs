use dioxus::prelude::*;
use dioxus_components_bin::{
    components::atom::separator::Separator, components::composite::scrollable::*,
    layout::docs::DocsLayout,
};

use crate::website::router::Route;

pub fn SideNavComp() -> Element {
    // TODO Find a better way to do this
    let atom_names = ["button", "icon", "placeholder", "separator", "typography"];

    let atoms = atom_names
        .iter()
        .map(|name| {
            rsx!(
                Link { to: format!("/component/atom/{}", name), {name} }
            )
        })
        .collect::<Vec<Element>>();

    let composite_names = [
        "accordion",
        "breadcrumb",
        "dropdown",
        "lightswitch",
        "modal",
        "navbar",
        "progressbar",
        "scrollable",
        "table",
        "tabs",
    ];

    let composites = composite_names
        .iter()
        .map(|name| {
            rsx!(
                Link { to: format!("/component/composite/{}", name), {name} }
            )
        })
        .collect::<Vec<Element>>();

    let form_names = [
        "checkbox",
        "formlist",
        "input",
        "radiogroup",
        "select",
        "slider",
        "textarea",
        "toggle",
    ];

    let forms = form_names
        .iter()
        .map(|name| {
            rsx!(
                Link { to: format!("/component/form/{}", name), {name} }
            )
        })
        .collect::<Vec<Element>>();

    rsx!(
        aside { class: "fixed top-14 z-30 -ml-2 hidden h-[calc(100vh-3.5rem)] w-full shrink-0 md:sticky md:block",
            Scrollable { class: "w-48 flex flex-col space-y-2 px-4",
                h5 { class: "h5", "ATOMS" }
                for atom in atoms {
                    div { class: "hover:underline", {atom} }
                }
                Separator {}
                h5 { class: "h5", "COMPOSITES" }
                for composite in composites {
                    div { class: "hover:underline", {composite} }
                }
                Separator {}
                h5 { class: "h5", "FORMS" }
                for form in forms {
                    div { class: "hover:underline", {form} }
                }
            }
        }
        div { class: "relative py-6 lg:gap-10 lg:py-8 xl:grid xl:grid-cols-[1fr_300px]",
            DocsLayout { Outlet::<Route> {} }
        }
    )
}
