use tailwind_fuse::*;

def_class_with_variant!(ButtonClass, r#"text-center
            text-base font-medium
            rounded-global-radius
            shadow-global-shadow
            transition-colors
            duration-150
            disabled:opacity-50 disabled:cursor-not-allowed"#, variant: ButtonVariant, color: ButtonColor, size: ButtonSize);

def_variant!(
    ButtonColor,
    Default => r#"bg-background text-foreground hover:bg-foreground/20 active:bg-foreground/30 active:shadow"#,
    Primary => r#"bg-primary text-primary-foreground border-primary hover:bg-primary/90 active:bg-primary/80 active:shadow"#,
    Secondary => r#"bg-secondary text-secondary-foreground border-secondary hover:bg-secondary/90 active:bg-secondary/80 active:shadow"#,
    Destructive => r#"bg-destructive text-destructive-foreground border-destructive hover:bg-destructive/90 active:bg-destructive/80 active:shadow"#
);

def_variant!(
    ButtonVariant,
    Default => r#"border-none"#,
    Outline => r#"border"#,
    Ghost => r#"border-none"#
);

def_variant!(
    ButtonSize,
    Md => r#"px-5 py-[9px] text-base"#,
    Sm => r#"px-3 py-2 text-sm"#,
    Lg => r#"px-6 py-3 text-lg"#,
    Xs => r#"px-2.5 py-1.5 text-xs"#,
    Xl => r#"px-8 py-4 text-xl"#
);
