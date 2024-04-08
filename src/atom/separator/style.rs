use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"bg-border shrink-0"#)]
pub struct SeparatorClass {
    vertical: SeparatorOrientation,
}

#[derive(TwVariant, PartialEq)]
pub enum SeparatorOrientation {
    #[tw(default, class = "w-full h-[1px]")]
    Horizontal,
    #[tw(class = "h-full w-[1px]")]
    Vertical,
}

impl From<bool> for SeparatorOrientation {
    fn from(value: bool) -> Self {
        match value {
            true => SeparatorOrientation::Vertical,
            false => SeparatorOrientation::Horizontal,
        }
    }
}
