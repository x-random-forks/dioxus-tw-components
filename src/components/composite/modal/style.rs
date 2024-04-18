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
        "p-medium flex flex-col top-[50%] left-[50%] z-50 bg-background border border-border rounded-global-radius fixed translate-x-[-50%] data-[state=inactive]:translate-y-full data-[state=inactive]:invisible data-[state=inactive]:opacity-0 data-[state=active]:translate-y-[-50%] transition-all duration-500"
    }
}

impl BaseClass for ModalBackgroundProps {
    fn base(&self) -> &'static str {
        "w-full h-full top-0 left-0 z-40 bg-[linear-gradient(_45deg,magenta,rebeccapurple,dodgerblue,green_)] opacity-75 data-[state=active]:fixed data-[state=inactive]:hidden transition-all"
    }
}
