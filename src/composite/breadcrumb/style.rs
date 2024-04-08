use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"flex flex-wrap items-center font-normal gap-2"#)]
pub struct BreadcrumbClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"font-normal"#)]
pub struct BreadcrumbItemClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"font-semibold"#)]
pub struct BreadcrumbSeparatorClass {}
