use dioxus::prelude::*;
use dioxus_components_bin::{
    atom::{icon::*, input::Input},
    composite::{lightswitch::*, navbar::*},
};
use dioxus_router::prelude::*;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(LightSwitchSignal("".to_string())));

    let light_switch_context = use_context::<Signal<LightSwitchSignal>>();
    let dark = &light_switch_context.read().0;
    rsx!(
        body { class: "{dark} bg-background", Router::<Route> {} }
    )
}

pub fn WrapperNav() -> Element {
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
    let left_part = rsx!("Dioxus Comp Lib");
    rsx!(
        Navbar { left_part: left_part, right_part: right_part }
        Outlet::<Route> {}
    )
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WrapperNav)]
        #[route("/")]
        Home,
}

pub fn Home() -> Element {
    rsx!( div { class: "h-screen w-screen", "Hello, World!" } )
}
