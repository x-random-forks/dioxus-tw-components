use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = "relative
    bg-input 
    rounded-full 
    peer 
    peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-input
    peer-checked:after:translate-x-full peer-checked:after:border-white
    after:content-[''] after:absolute after:bg-white after:border-input after:border after:rounded-full after:transition-all")]
pub struct ToggleClass {
    pub color: ToggleColor,
    pub size: ToggleSize,
}

#[derive(TwVariant, PartialEq)]
pub enum ToggleColor {
    #[tw(default, class = "peer-checked:bg-primary")]
    Primary,
    #[tw(class = "peer-checked:bg-secondary")]
    Secondary,
    #[tw(class = "peer-checked:bg-destructive")]
    Destructive,
}

#[derive(TwVariant, PartialEq)]
pub enum ToggleSize {
    #[tw(class = "w-14 h-7 after:top-0.5 after:start-[4px] after:h-6 after:w-6")]
    Lg,
    #[tw(
        default,
        class = "w-11 h-6 after:top-[2px] after:start-[2px] after:h-5 after:w-5"
    )]
    Md,
    #[tw(class = "w-9 h-5 after:top-[2px] after:start-[2px] after:h-4 after:w-4")]
    Sm,
}
