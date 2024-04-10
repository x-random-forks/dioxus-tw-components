use tailwind_fuse::*;

// def_class_no_variant!(AccordionClass, "");

def_class_no_variant!(AccordionItemClass, "border-b");

def_class_no_variant!(
    AccordionTriggerClass,
    "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline"
);

def_class_no_variant!(
    AccordionContentClass,
    "text-sm overflow-hidden transition-all"
);
