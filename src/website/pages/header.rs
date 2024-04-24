use dioxus::prelude::*;
use dioxus_components::{
    components::composites::{lightswitch::*, navbar::*},
    components::molecules::form::input::Input,
    layout::{HeaderLayout, MainLayout},
};

use crate::website::router::Route;

pub fn Header() -> Element {
    rsx!(
        HeaderLayout { Navbar {
            div { class: "flex flex-1 space-x-2 items-center ml-6",
                Link { class: "mr-6", to: "/",
                    div { class: "text-foreground font-bold", "Dioxus Lib Comp" }
                }
                ul { class: "text-sm list-none flex space-x-2",
                    li {
                        Link { class: "anchor", to: "/components/atoms/button", "Components" }
                    }
                    li {
                        Link { class: "anchor", to: "/layouts/tmp", "Layouts" }
                    }
                }
            }
            div { class: "flex flex-1 items-center justify-end space-x-2 mr-6",
                div { class: "w-48",
                    Input { r#type: "search", placeholder: "Search" }
                }
                a {
                    class: "flex justify-center items-center size-10 rounded-global-radius p-2 transition-colors hover:bg-foreground/30",
                    href: "https://github.com/42Angouleme/SSCCE_dioxus",
                    target: "_blank",
                    dioxus_free_icons::Icon {
                        class: "",
                        width: 24,
                        height: 24,
                        fill: "currentColor",
                        icon: dioxus_free_icons::icons::fa_brands_icons::FaGithub
                    }
                }
                LightSwitch { class: "size-10 p-2 flex items-center justify-center rounded-global-radius transition-colors hover:bg-foreground/30" }
            }
        } }
        MainLayout { Outlet::<Route> {} }
    )
}
