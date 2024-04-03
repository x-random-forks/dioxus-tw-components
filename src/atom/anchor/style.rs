use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"
    text-blue-500
    hover:text-blue-700"#)]
pub struct AnchorClass {}
