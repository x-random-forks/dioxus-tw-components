use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct ProgressTrackProps {
    children: Element,
    // Styling
    #[props(default)]
    color: ProgressTrackColor,
    #[props(default)]
    size: ProgressTrackSize,
    #[props(default)]
    class: String,
}

impl Component for ProgressTrackProps {
    fn view(self) -> Element {
        let class = ProgressTrackClass::builder()
            .color(self.color)
            .size(self.size)
            .with_class(self.class);

        rsx!(
            div { class: "{class}", {self.children} }
        )
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct ProgressBarProps {
    #[props(default = 50)]
    progress: u8,
    children: Element,
    // Styling
    #[props(default)]
    color: ProgressBarColor,
    #[props(default)]
    class: String,
}

impl Component for ProgressBarProps {
    fn view(self) -> Element {
        let class = ProgressBarClass::builder()
            .color(self.color)
            .with_class(self.class);

        rsx!(
            div { class: "{class}", style: "width: {self.progress}%",
                div { {self.children} }
            }
        )
    }
}

#[derive(PartialEq, Props, Clone, Component)]
pub struct ProgressLabelProps {
    #[props(default = 50)]
    progress: u8,
    #[props(default = true)]
    show_percentage: bool,
    // Styling
    #[props(default)]
    color: ProgressLabelColor,
    #[props(default)]
    class: String,
}

impl Component for ProgressLabelProps {
    fn view(self) -> Element {
        let class = ProgressLabelClass::builder()
            .color(self.color)
            .with_class(self.class);

        rsx!(
            span { class: "{class}",
                {self.progress.to_string()},
                if self.show_percentage {
                    "%"
                }
            }
        )
    }
}
