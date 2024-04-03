use dioxus::prelude::*;
use dioxus_components_bin::{
    atom::{icon::*, input::*},
    composite::{lightswitch::*, navbar::*},
};

use crate::website::router::Route;

pub fn Header() -> Element {
    let right_part = rsx!(
        div { class: "w-48", Input { r#type: "search", placeholder: "Search" } }
        div { class: "size-10 rounded-global-radius p-2 transition-colors hover:bg-foreground/30",
            a {
                href: "https://github.com/42Angouleme/SSCCE_dioxus",
                target: "_blank",
                Icon { svg: GitHub }
            }
        }
        LightSwitch { class: "size-10 p-2 flex items-center justify-center rounded-global-radius transition-colors hover:bg-foreground/30" }
    );

    let left_part = rsx!(
        Link { to: "/", div { class: "text-foreground font-bold", "Dioxus Lib Comp" } }
        Link { class: "anchor", to: "/component/atom/button", "Component" }
    );

    let header_class = "sticky w-full top-0 left-0 z-30 border-border border-b backdrop-filter backdrop-blur bg-background/80 overflow-y-hidden items-center justify-between";

    rsx!(
        header { class: "{header_class}", Navbar { left_part: left_part, right_part: right_part } }
        main { class: "flex-1",
            div { class: "relative container", Outlet::<Route> {} }
        }
    )
}
