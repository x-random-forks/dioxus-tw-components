use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"
            flex w-full
            h-8
            px-3 py-2
            text-sm text-foreground
            bg-background
            border border-input
            rounded-global-radius
            hover:brightness-105
            focus:outline-none
            focus:brightness-105 focus:ring-ring focus:ring-offset-2 focus:ring-2
            disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100
            file:font-medium file:bg-input file:rounded-global-radius file:border-0"#)]
pub struct InputClass {}
