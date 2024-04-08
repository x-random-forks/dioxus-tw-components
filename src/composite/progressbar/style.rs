use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = "w-full rounded-global-radius")]
pub struct ProgressTrackClass {
    color: ProgressTrackColor,
    size: ProgressTrackSize,
}

#[derive(TwVariant, PartialEq)]
pub enum ProgressTrackColor {
    #[tw(class = "bg-primary")]
    Primary,
    #[tw(default, class = "bg-input")]
    Secondary,
    #[tw(class = "bg-destructive")]
    Destructive,
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
#[tw(class = "h-full rounded-global-radius flex items-center justify-center")]
pub struct ProgressBarClass {
    color: ProgressBarColor,
}

#[derive(TwVariant, PartialEq)]
pub enum ProgressBarColor {
    #[tw(default, class = "bg-primary [&>*]:text-primary-foreground")]
    Primary,
    #[tw(class = "bg-secondary [&>*]:text-secondary-foreground")]
    Secondary,
    #[tw(class = "bg-destructive [&>*]:text-destructive-foreground")]
    Destructive,
}

// By default the color is set by ProgressBar using [&>*] selector, you can override it by passing
// your color of choice to ProgressLabel's class
#[derive(TwClass, Clone, Copy)]
#[tw(class = "")]
pub struct ProgressLabelClass {}
