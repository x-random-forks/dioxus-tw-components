use tailwind_fuse::*;

def_class_with_variant!(
    ProgressTrackClass,
    r#"w-full rounded-global-radius"#,
    color: ProgressTrackColor,
    size: ProgressTrackSize,
);

def_variant!(
    ProgressTrackColor,
    Primary => r#"bg-primary"#,
    Secondary => r#"bg-input"#,
    Destructive => r#"bg-destructive"#
);

def_variant!(
    ProgressTrackSize,
    Md => r#"h-4 text-xs"#,
    Sm => r#"h-2 text-xs"#,
    Lg => r#"h-6 text-base"#,
    Xl => r#"h-8 text-lg"#
);

def_class_with_variant!(
    ProgressBarClass,
    r#"h-full rounded-global-radius flex items-center justify-center"#,
    color: ProgressBarColor,
);

def_variant!(
    ProgressBarColor,
    Primary => r#"bg-primary [&>*]:text-primary-foreground"#,
    Secondary => r#"bg-secondary [&>*]:text-secondary-foreground"#,
    Destructive => r#"bg-destructive [&>*]:text-destructive-foreground"#
);

// By default the color is set by ProgressBar using [&>*] selector, you can override it by passing
// your color of choice to ProgressLabel's class
def_class_no_variant!(ProgressLabelClass, r#""#);
