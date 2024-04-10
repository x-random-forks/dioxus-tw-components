use tailwind_fuse::*;

def_class_with_variant!(CheckboxClass, r#"peer"#, color: CheckboxColor);

def_variant!(
    CheckboxColor,
    Primary => r#"accent-primary focus:ring-primary focus:ring-2 focus:ring-offset-1"#,
    Secondary => r#"accent-secondary focus:ring-secondary focus:ring-2 focus:ring-offset-1"#,
    Accent => r#"accent-accent focus:ring-accent-foreground focus:ring-2 focus:ring-offset-1"#
);
