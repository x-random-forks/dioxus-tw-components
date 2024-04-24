use dioxus::prelude::*;
use dioxus_components::{components::composites::scrollable::*, layout::Docs};

use crate::website::router::Route;

pub fn SideNavLayouts() -> Element {
    let layout_names = ["tmp"];

    let layouts = layout_names
        .iter()
        .map(|name| {
            rsx!(
                Link { to: format!("/layouts/{}", name), {name} }
            )
        })
        .collect::<Vec<Element>>();

    rsx!(
        aside { class: "fixed top-14 z-30 -ml-2 hidden h-[calc(100vh-3.5rem)] w-full shrink-0 md:sticky md:block",
            Scrollable { class: "w-48 flex flex-col space-y-2 px-4",
                h5 { class: "h5", "LAYOUTS" }
                for layout in layouts {
                    div { class: "hover:underline", {layout} }
                }
            }
        }
        div { class: "relative py-6 lg:gap-10 lg:py-8 xl:grid xl:grid-cols-[1fr_300px]",
            Docs { Outlet::<Route> {} }
        }
    )
}
