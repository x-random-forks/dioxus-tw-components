use tailwind_fuse::*;

// Used to make a "useless" div which does not create a newline that wrap our trigger with our trigger_closure
// Also used by ModalCancelProps
#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"inline-block"#)]
pub struct ModalTriggerClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(class = r#"absolute top-2 right-2"#)]
pub struct ModalCancelClass {}

// The actual modal
#[derive(TwClass, Clone, Copy)]
#[tw(
    class = r#"top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] z-50 bg-background border-border rounded-global-radius p-4 flex flex-col"#
)]
pub struct ModalContentClass {}

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = r#"w-full h-full top-0 left-0 z-40 bg-[linear-gradient(_45deg,magenta,rebeccapurple,dodgerblue,green_)] opacity-75"#
)]
pub struct ModalBackgroundClass {}
