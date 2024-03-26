use self::styling::Color;
use super::style::IconSvg;
use crate::*;
use component_derive::Component;

pub use Color::{Accent, Default, Primary, Secondary};

#[derive(PartialEq, Props, Clone, Component)]
pub struct IconProps {
    // Namespace of the svg
    #[props(default = "http://www.w3.org/2000/svg".to_string())]
    xmlns: String,
    // Icon type
    #[props(default = IconSvg::HollowCircle)]
    svg: IconSvg,
    #[props(default = Color::Primary)]
    color: Color,
}

impl Component for IconProps {
    fn view(self) -> Element {
        let fill_color = match self.color {
            Color::Default => "fill-foreground",
            Color::Primary => "fill-primary",
            Color::Secondary => "fill-secondary",
            Color::Accent => "fill-accent",
            _ => "fill-none",
        };
        let stroke_color = match self.color {
            Color::Default => "stroke-foreground",
            Color::Primary => "stroke-primary",
            Color::Secondary => "stroke-secondary",
            Color::Accent => "stroke-accent",
            _ => "stroke-none",
        };

        let svg = match self.svg {
            IconSvg::HollowCircle => rsx!(circle {
                class: "{stroke_color} fill-none stroke-1",
                cx: "12",
                cy: "12",
                r: "10"
            }),
            IconSvg::CircleInnerCircle => rsx!(
                circle {
                    class: "{stroke_color} fill-none stroke-1",
                    cx: "12",
                    cy: "12",
                    r: "10"
                }
                circle { class: "{fill_color}", cx: "12", cy: "12", r: "6" }
            ),
        };

        rsx!(
            svg {
                // class:"fill-none stroke-primary stroke-1",
                xmlns: "{self.xmlns}",
                // width: "24",
                // height: "24",
                view_box: "0 0 24 24",
                stroke_linecap: "round",
                {svg}
            }
        )
    }
}

// <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#9b1717"
//  stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-circle"><circle cx="12" cy="12" r="10"/></svg>
