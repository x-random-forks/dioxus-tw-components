use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = "peer")]
pub struct CheckboxClass {
    pub color: CheckboxColor,
}

#[derive(TwVariant, PartialEq)]
pub enum CheckboxColor {
    #[tw(
        default,
        class = "accent-primary focus:ring-primary focus:ring-2 focus:ring-offset-1"
    )]
    Primary,
    #[tw(class = "accent-secondary focus:ring-secondary focus:ring-2 focus:ring-offset-1")]
    Secondary,
    #[tw(class = "accent-accent focus:ring-accent-foreground focus:ring-2 focus:ring-offset-1")]
    Accent,
}
