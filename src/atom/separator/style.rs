use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"bg-border shrink-0"#)]
pub struct SeparatorClass {
    vertical: SeparatorOrientation,
    color: SeparatorColor,
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

#[derive(TwVariant, PartialEq)]
pub enum SeparatorColor {
    #[tw(default, class = "bg-border")]
    Border,
    #[tw(class = "bg-primary")]
    Primary,
    #[tw(class = "bg-secondary")]
    Secondary,
    #[tw(class = "bg-destructive")]
    Destructive,
}
