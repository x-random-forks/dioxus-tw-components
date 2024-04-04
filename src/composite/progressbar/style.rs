use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"w-full rounded-global-radius bg-secondary h-4"#)]
pub struct ProgressTrackClass {
    color: ProgressTrackColor,
    size: ProgressTrackSize,
}

#[derive(TwVariant, PartialEq)]
pub enum ProgressTrackColor {
    #[tw(class = "bg-primary")]
    Primary,
    #[tw(default, class = "bg-secondary")]
    Secondary,
    #[tw(class = "bg-accent")]
    Accent,
}

#[derive(TwVariant, PartialEq)]
pub enum ProgressTrackSize {
    #[tw(class = "h-2")]
    Sm,
    #[tw(default, class = "h-4")]
    Md,
    #[tw(class = "h-6")]
    Lg,
    #[tw(class = "h-8")]
    Xl,
}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"h-full rounded-global-radius"#)]
pub struct ProgressBarClass {
    color: ProgressBarColor,
}

#[derive(TwVariant, PartialEq)]
pub enum ProgressBarColor {
    #[tw(default, class = "bg-primary")]
    Primary,
    #[tw(class = "bg-secondary")]
    Secondary,
    #[tw(class = "bg-accent")]
    Accent,
}
