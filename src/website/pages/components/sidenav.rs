use dioxus::prelude::*;
use dioxus_components_bin::{
    atom::separator::Separator, composite::scrollable::*, layout::docslayout::DocsLayout,
};

use crate::website::router::Route;

pub fn SideNavComp() -> Element {
    // TODO Find a better way to do this
    let atom_names = [
        "button",
        "checkbox",
        "code",
        "formrange",
        "icon",
        "input",
        "label",
        "separator",
        "textarea",
        "toggle",
        "typography",
    ];

    let atoms = atom_names
        .iter()
        .map(|name| {
            rsx!(
                Link { to: format!("/component/atom/{}", name), {name} }
            )
        })
        .collect::<Vec<Element>>();

    let composite_names = [
        "breadcrumb",
        "dropdown",
        "formlist",
        "lightswitch",
        "modal",
        "navbar",
        "progressbar",
        "radiogroup",
        "scrollable",
        "select",
    ];

    let composites = composite_names
        .iter()
        .map(|name| {
            rsx!(
                Link { to: format!("/component/composite/{}", name), {name} }
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
            }
        }
        div { class: "relative py-6 lg:gap-10 lg:py-8 xl:grid xl:grid-cols-[1fr_300px]",
            DocsLayout { Outlet::<Route> {} }
        }
    )
}
