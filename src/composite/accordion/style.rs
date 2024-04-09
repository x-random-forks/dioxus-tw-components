use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#""#)]
pub struct AccordionClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"border-b"#)]
pub struct AccordionItemClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = r#"flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline"#
)]
pub struct AccordionTriggerClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"text-sm overflow-hidden transition-all"#)]
pub struct AccordionContentClass {}
