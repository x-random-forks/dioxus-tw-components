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
                #[route("/:name")]
                ComponentPage {name: String},
            #[end_layout]
        #[end_nest]

        #[route("/:..route")]
        NotFound {route: Vec<String>}
}   

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx!( "Not found" )
}