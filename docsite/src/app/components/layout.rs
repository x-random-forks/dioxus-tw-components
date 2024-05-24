use dioxus::prelude::*;
use dioxus_components::molecules::Scrollable;

use crate::app::router::Route;

#[component]
pub fn SideBarComponent() -> Element {
    rsx!(
        ComponentPage { 
            SideBarTemplate { "NAVBAR" }
            DocTemplate { Outlet::<Route> {} }
        }
    )
}

#[component]
pub fn SideBarTemplate(children: Element) -> Element {
    rsx!(
        aside { class: "max-w-[220px] border border-black", Scrollable { {children} } }
    )
}

#[component]
pub fn ComponentPage(children: Element) -> Element {
    rsx!(
        div { class: "grid grid-cols-[220px_minmax(0,1fr)] max-w-screen-xl border border-primary",
            {children}
        }
    )
}


#[component]
pub fn DocTemplate(children: Element) -> Element {
    rsx!(
        div { class: "border border-secondary", {children} }
    )
}