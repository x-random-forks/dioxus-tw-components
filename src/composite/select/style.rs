use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"flex flex-col w-full bg-input"#)]
pub struct SelectGroupClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"hidden"#)]
pub struct SelectPlaceholderClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"px-2 py-1.5 text-sm font-semibold"#)]
pub struct SelectLabelClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"flex py-1.5 pl-2 pr-8 text-sm"#)]
pub struct SelectItemClass {}
