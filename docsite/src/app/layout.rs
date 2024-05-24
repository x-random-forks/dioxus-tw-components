use dioxus::prelude::*;
use dioxus_components::{
    form::Input,
    molecules::{LightSwitch, Navbar},
    templates::HeaderTemplate,
};

use crate::app::router::Route;

#[component]
pub fn Header() -> Element {
    rsx!(
        HeaderTemplate { 
            Navbar { class: "",
                div { class: "flex flex-1 space-x-2 items-center ml-6",
                    Link { class: "mr-6", to: "/",
                        div { class: "text-foreground font-bold", "Dioxus Lib Comp" }
                    }
                    ul { class: "text-sm list-none flex space-x-2",
                        li {
                            Link { class: "anchor", to: "/components/atoms/button", "Components" }
                        }
                    }
                }
                div { class: "flex flex-1 items-center justify-end space-x-2 mr-6",
                    div { class: "w-48",
                        Input { r#type: "search", placeholder: "Search WIP" }
                    }
                    a {
                        class: "flex justify-center items-center size-10 rounded-global-radius p-2 transition-colors hover:bg-foreground/30",
                        href: "https://github.com/42Angouleme/SSCCE_dioxus",
                        target: "_blank",
                        "GIT"
                    }
                    LightSwitch { class: "size-10 p-2 flex items-center justify-center rounded-global-radius transition-colors hover:bg-foreground/70" }
                }
            }
        }
        main { class: "min-h-screen border border-blue-500", Outlet::<Route> {} }
    )
}
