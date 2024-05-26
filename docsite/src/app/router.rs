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
        #[layout(SideBarComponent)]
            #[route("/components/atoms/button")]
            ButtonPage {},
            #[route("/components/atoms/placeholder")]
            PlaceholderPage {}
}   