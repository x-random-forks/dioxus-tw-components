use super::props::*;
use crate::attributes::*;

// Used to make a "useless" div which does not create a newline that wrap our trigger with our trigger_closure
// Also used by ModalCancelProps
impl Class for ModalTriggerProps {
    fn base(&self) -> &'static str {
        "inline-block"
    }
}

impl Class for ModalCloseProps {
    fn base(&self) -> &'static str {
        "absolute top-2 right-2"
    }
}

impl Class for ModalContentProps {
    fn base(&self) -> &'static str {
        "p-medium flex flex-col top-[50%] left-[50%] z-50 bg-background border border-border rounded-global-radius fixed translate-x-[-50%] translate-y-[-50%] data-[state=inactive]:invisible"
    }
    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::None => "",
            Animation::Light | Animation::Full => "data-[state=inactive]:translate-y-full data-[state=inactive]:opacity-0 transition-all duration-300",
            Animation::Custom(animation) => animation,
        })
    }
}

impl Class for ModalBackgroundProps {
    fn base(&self) -> &'static str {
        "w-full h-full top-0 left-0 z-40 opacity-75 fixed data-[state=inactive]:invisible bg-[linear-gradient(_45deg,magenta,rebeccapurple,dodgerblue,green_)]"
    }
    
    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::None => "",
            Animation::Light | Animation::Full => "data-[state=inactive]:opacity-0 data-[state=inactive]:invisible transition-all duration-300",
            Animation::Custom(animation) => animation,
        })
    }
}
