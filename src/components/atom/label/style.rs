use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"font-medium  peer-disabled:text-muted"#)]
pub struct LabelClass {
    pub color: LabelColor,
}

#[derive(TwVariant, PartialEq)]
pub enum LabelColor {
    #[tw(class = "text-primary")]
    Primary,
    #[tw(class = "text-secondary")]
    Secondary,
    #[tw(class = "text-accent-foreground")]
    Accent,
    #[tw(default, class = "text-foreground")]
    Default,
}
