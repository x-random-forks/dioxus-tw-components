use dioxus::prelude::*;
use dioxus_components_bin::{
    atom::{icon::*, input::*},
    composite::{lightswitch::*, navbar::*},
    layout::{header::HeaderLayout, mainlayout::MainLayout},
};

use crate::website::router::Route;

pub fn Header() -> Element {
    let left_part = rsx!(
        Link { class: "mr-6", to: "/", div { class: "text-foreground font-bold", "Dioxus Lib Comp" } }
        ul { class: "text-sm list-none flex space-x-2",
            li {
                Link { class: "anchor", to: "/component/atom/button", "Component" }
            }
        }
    );

    let right_part = rsx!(
        div { class: "w-48", Input { r#type: "search", placeholder: "Search" } }
        a {
            class: "size-10 rounded-global-radius p-2 transition-colors hover:bg-foreground/30",
            href: "https://github.com/42Angouleme/SSCCE_dioxus",
            target: "_blank",
            Icon { svg: GitHub }
        }
        LightSwitch { class: "size-10 p-2 flex items-center justify-center rounded-global-radius transition-colors hover:bg-foreground/30" }
    );

    rsx!(
        HeaderLayout { Navbar { left_part: left_part, right_part: right_part } }
        MainLayout { Outlet::<Route> {} }
    )
}
