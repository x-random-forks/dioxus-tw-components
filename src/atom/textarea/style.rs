use tailwind_fuse::*;

def_class_no_variant!(
    TextAreaClass,
    r#"
    flex w-full
    px-3 py-2
    text-sm text-foreground
    bg-background
    border border-input
    rounded-global-radius
    hover:brightness-105
    focus:brightness-105
    disabled:bg-muted disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100"#
);
