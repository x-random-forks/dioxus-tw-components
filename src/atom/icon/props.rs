use self::styling::Color;
use super::style::IconSvg;
use crate::*;
use component_derive::Component;

pub use Color::{Accent, DefaultColor, Primary, Secondary, Unset};
pub use IconSvg::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct IconProps {
    // Namespace of the svg
    #[props(default = "http://www.w3.org/2000/svg".to_string())]
    xmlns: String,
    // Icon type
    #[props(default = IconSvg::HollowCircle)]
    svg: IconSvg,
    #[props(default = Color::default())]
    color: Color,
}

// TODO
impl Component for IconProps {
    fn view(self) -> Element {
        let fill_color = match self.color {
            Color::DefaultColor => "fill-foreground",
            Color::Primary => "fill-primary",
            Color::Secondary => "fill-secondary",
            Color::Accent => "fill-accent",
            _ => "fill-none",
        };
        let stroke_color = match self.color {
            Color::DefaultColor => "stroke-foreground",
            Color::Primary => "stroke-primary",
            Color::Secondary => "stroke-secondary",
            Color::Accent => "stroke-accent",
            _ => "stroke-none",
        };

        let svg = match self.svg {
            IconSvg::HollowCircle => rsx!(
                circle {
                    class: "{stroke_color} fill-none stroke-1",
                    cx: "12",
                    cy: "12",
                    r: "10"
                }
            ),
            IconSvg::CircleInnerCircle => rsx!(
                circle {
                    class: "{stroke_color} fill-none stroke-1",
                    cx: "12",
                    cy: "12",
                    r: "10"
                }
                circle { class: "{fill_color}", cx: "12", cy: "12", r: "6" }
            ),
            IconSvg::GitHub => rsx!(
                path {
                    class: "{fill_color}",
                    d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                }
            )
        };

        rsx!(
            svg {
                xmlns: "{self.xmlns}",
                view_box: "0 0 24 24",
                stroke_linecap: "round",
                {svg}
            }
        )
    }
}

// <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#9b1717"
//  stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-circle"><circle cx="12" cy="12" r="10"/></svg>
