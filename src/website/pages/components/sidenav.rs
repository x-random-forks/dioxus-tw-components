use dioxus::prelude::*;
use dioxus_components_bin::{atom::separator::Separator, composite::scrollable::*};

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

    // let test = use_route::<Route>().to_string();
    // log::debug!("test: {}", test);

    rsx!(
        aside { class: "flex",
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
            div { class: "flex-1 p-10 text-2xl font-bold", Outlet::<Route> {} }
        }
    )
}
