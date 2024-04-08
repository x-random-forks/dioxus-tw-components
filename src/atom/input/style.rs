use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"
            flex w-full
            px-3 py-1
            text-foreground
            bg-background
            border border-input
            rounded-global-radius
            hover:brightness-105
            focus:outline-none
            focus:brightness-105 focus:ring-ring focus:ring-offset-2 focus:ring-2
            disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100
            file:font-medium file:bg-input file:rounded-sm file:border-0 file:items-center file:justify-center"#)]
pub struct InputClass {
    size: InputSize,
}

#[derive(TwVariant, PartialEq)]
pub enum InputSize {
    #[tw(default, class = "h-8 text-sm")]
    Md,
    #[tw(class = "h-10 text-base")]
    Lg,
    #[tw(class = "h-6 text-xs")]
    Sm,
    #[tw(class = "h-12 text-lg")]
    Xl,
}
