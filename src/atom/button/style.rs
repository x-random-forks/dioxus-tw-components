use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"text-center
            text-base font-medium
            rounded-global-radius
            shadow-global-shadow
            transition-colors
            duration-150
            disabled:opacity-50 disabled:cursor-not-allowed"#)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(TwVariant, PartialEq)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Primary,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/90")]
    Secondary,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "border-none text-foreground hover:bg-accent")]
    Ghost,
}

#[derive(TwVariant, PartialEq)]
pub enum ButtonSize {
    #[tw(default, class = "px-5 py-[9px] text-base")]
    Md,
    #[tw(class = "px-3 py-2 text-sm")]
    Sm,
    #[tw(class = "px-6 py-3 text-lg")]
    Lg,
    #[tw(class = "px-2.5 py-1.5 text-xs")]
    Xs,
    #[tw(class = "px-8 py-4 text-xl")]
    Xl,
}
