use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"w-full rounded-global-radius"#)]
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
    #[tw(class = "h-2 text-xs")]
    Sm,
    #[tw(default, class = "h-4 text-xs")]
    Md,
    #[tw(class = "h-6 text-base")]
    Lg,
    #[tw(class = "h-8 text-lg")]
    Xl,
}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"h-full rounded-global-radius flex items-center justify-center"#)]
pub struct ProgressBarClass {
    color: ProgressBarColor,
}

#[derive(TwVariant, PartialEq)]
pub enum ProgressBarColor {
    #[tw(default, class = "bg-primary [&>*]:text-primary-foreground")]
    Primary,
    #[tw(class = "bg-secondary [&>*]:text-secondary-foreground")]
    Secondary,
    #[tw(class = "bg-accent [&>*]:text-accent-foreground")]
    Accent,
}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#""#)]
pub struct ProgressLabelClass {
    color: ProgressLabelColor,
}

// By default the color is set by ProgressBar using [&>*] selector
#[derive(TwVariant, PartialEq)]
pub enum ProgressLabelColor {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "text-primary-foreground")]
    Primary,
    #[tw(class = "text-secondary-foreground")]
    Secondary,
    #[tw(class = "text-accent-foreground")]
    Accent,
}
