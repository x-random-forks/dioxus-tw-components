use dioxus::prelude::*;

use super::home::HomePage;
use super::layout::Header;
use super::components::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        HomePage {},
        #[nest("/components")]
            #[layout(SideBarComponent)]
                #[nest("/atoms")]
                #[route("/button")]
                ButtonPage {},
                #[route("/buttongroup")]
                ButtonGroupPage {},
                #[route("/placeholder")]
                PlaceholderPage {},
                #[end_nest]
                
                #[nest("/molecules")]
                #[route("/tabs")]
                TabsPage {},
                #[end_nest]
            #[end_layout]
        #[end_nest]

        #[route("/:..route")]
        NotFound {route: Vec<String>}
}   

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx!( "Not found" )
}