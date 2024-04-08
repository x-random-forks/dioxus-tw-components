use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"bg-muted"#)]
pub struct PlaceholderClass {
    radius: PlaceholderRadius,
    animation: PlaceholderAnimation,
}

#[derive(TwVariant, PartialEq)]
pub enum PlaceholderRadius {
    #[tw(default, class = "rounded-global-radius")]
    Global,
    #[tw(class = "rounded-full")]
    Full,
    #[tw(class = "rounded-none")]
    None,
}

#[derive(TwVariant, PartialEq)]
pub enum PlaceholderAnimation {
    #[tw(default, class = "animate-pulse")]
    Pulse,
    #[tw(class = "")]
    None,
}

impl From<bool> for PlaceholderAnimation {
    fn from(animated: bool) -> Self {
        match animated {
            true => PlaceholderAnimation::Pulse,
            false => PlaceholderAnimation::None,
        }
    }
}
