use tailwind_fuse::*;

def_class_with_variant!(
    ToggleClass,
    r#"
    relative
    bg-input 
    rounded-full 
    peer 
    peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-input
    peer-checked:after:translate-x-full peer-checked:after:border-white
    after:content-[''] after:absolute after:bg-white after:border-input after:border after:rounded-full after:transition-all"#,
    color: ToggleColor, size: ToggleSize);

def_variant!(
    ToggleColor,
    Primary => r#"peer-checked:bg-primary"#,
    Secondary => r#"peer-checked:bg-secondary"#,
    Destructive => r#"peer-checked:bg-destructive"#
);

def_variant!(
    ToggleSize,
    Lg => r#"w-14 h-7 after:top-0.5 after:start-[4px] after:h-6 after:w-6"#,
    Md => r#"w-11 h-6 after:top-[2px] after:start-[2px] after:h-5 after:w-5"#,
    Sm => r#"w-9 h-5 after:top-[2px] after:start-[2px] after:h-4 after:w-4"#
);
