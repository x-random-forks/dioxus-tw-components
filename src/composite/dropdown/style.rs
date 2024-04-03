use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"relative group"#)]
pub struct DropdownClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#""#)]
pub struct DropdownToggleClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = r#"absolute hidden z-10 group-focus-within:block shadow-sm bg-background rounded-global-radius border border-border my-1 p-2 whitespace-nowrap"#
)]
pub struct DropdownContentClass {}
