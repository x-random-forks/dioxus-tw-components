use super::SeparatorProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<SeparatorProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => "bg-border shrink-0",
            _ => "",
        };
        write!(f, "{}", class)
    }
}

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
