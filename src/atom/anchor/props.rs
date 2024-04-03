use self::styling::BaseClass;
use crate::*;

use component_derive::Component;

#[derive(PartialEq, Props, Clone, Component)]
pub struct AnchorProps {
    #[props(default = "#".to_string())]
    href: String,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for AnchorProps {
    fn view(self) -> Element {
        let class = class!(BaseClass::<AnchorProps>::BaseClass, self.class);
        rsx!( a { class: "{class}" } )
    }
}
