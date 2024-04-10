use tailwind_fuse::*;

def_class_with_variant!(ButtonClass, r#"text-center
            text-base font-medium
            rounded-global-radius
            shadow-global-shadow
            transition-colors
            duration-150
            disabled:opacity-50 disabled:cursor-not-allowed"#, variant: ButtonVariant, size: ButtonSize);

def_variant!(
    ButtonVariant,
    Primary => r#"bg-primary text-primary-foreground hover:bg-primary/90 active:bg-primary/80 active:shadow"#,
    Secondary => r#"bg-secondary text-secondary-foreground hover:bg-secondary/90 active:bg-secondary/80 active:shadow"#,
    Outline => r#"border border-input bg-background hover:bg-accent/80 hover:text-accent-foreground active:bg-accent active:shadow"#,
    Ghost => r#"border-none text-foreground hover:bg-accent/80 active:bg-accent active:shadow"#
);

def_variant!(
    ButtonSize,
    Md => r#"px-5 py-[9px] text-base"#,
    Sm => r#"px-3 py-2 text-sm"#,
    Lg => r#"px-6 py-3 text-lg"#,
    Xs => r#"px-2.5 py-1.5 text-xs"#,
    Xl => r#"px-8 py-4 text-xl"#
);
