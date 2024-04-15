use super::props::*;
use crate::types::*;

// Used to make a "useless" div which does not create a newline that wrap our trigger with our trigger_closure
// Also used by ModalCancelProps
impl BaseClass for ModalTriggerProps {
    fn base(&self) -> &'static str {
        "inline-block"
    }
}

impl BaseClass for ModalCloseProps {
    fn base(&self) -> &'static str {
        "absolute top-2 right-2"
    }
}

impl BaseClass for ModalContentProps {
    fn base(&self) -> &'static str {
        "top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] z-50 bg-background border-border rounded-global-radius p-4 flex flex-col"
    }
}

// TODO : Probably add variant to define basic background colors ?
impl BaseClass for ModalBackgroundProps {
    fn base(&self) -> &'static str {
        "w-full h-full top-0 left-0 z-40 bg-[linear-gradient(_45deg,magenta,rebeccapurple,dodgerblue,green_)] opacity-75"
    }
}
