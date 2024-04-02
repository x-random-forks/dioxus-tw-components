use dioxus::prelude::*;
use dioxus_components_bin::{atom::separator::Separator, composite::scrollable::*};

use crate::website::router::Route;

pub fn SideNavComp() -> Element {
    // TODO Find a better way to do this
    let atom_names = [
        "button",
        "checkbox",
        "formrange",
        "icon",
        "input",
        "label",
        "separator",
        "textarea",
        "toggle",
    ];

    let atoms = atom_names
        .iter()
        .map(|name| {
            rsx!(
                Link { to: format!("/component/atom/{}", name), {name} }
            )
        })
        .collect::<Vec<Element>>();

    let composite_names = ["lightswitch", "scrollable"];
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
        aside { class: "flex fixed",
            Scrollable { class: "w-48 bg-accent",
                h4 { "ATOMS" }
                for atom in atoms {
                    div { class: "", {atom} }
                }
                Separator {}
                h4 { "COMPOSITES" }
                for composite in composites {
                    div { {composite} }
                }
            }
            div { class: "flex-1 p-10 text-2xl font-bold", Outlet::<Route> {} }
        }
    )
}
