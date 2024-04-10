use tailwind_fuse::*;

// Used to make a "useless" div which does not create a newline that wrap our trigger with our trigger_closure
// Also used by ModalCancelProps
def_class_no_variant!(ModalTriggerClass, r#"inline-block"#);

def_class_no_variant!(ModalCancelClass, r#"absolute top-2 right-2"#);

def_class_no_variant!(
    ModalContentClass,
    r#"top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] z-50 bg-background border-border rounded-global-radius p-4 flex flex-col"#
);

// TODO : Probably add variant to define basic background colors ?
def_class_no_variant!(
    ModalBackgroundClass,
    r#"w-full h-full top-0 left-0 z-40 bg-[linear-gradient(_45deg,magenta,rebeccapurple,dodgerblue,green_)] opacity-75"#
);
