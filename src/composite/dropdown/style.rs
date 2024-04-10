use tailwind_fuse::*;

def_class_no_variant!(DropdownClass, r#"relative group"#);

def_class_no_variant!(DropdownToggleClass, r#""#);

def_class_no_variant!(
    DropdownContentClass,
    r#"absolute hidden z-10 group-focus-within:block shadow-sm bg-background rounded-global-radius border border-border my-1 p-2 whitespace-nowrap"#
);
