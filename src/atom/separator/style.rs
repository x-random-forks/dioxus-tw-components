use tailwind_fuse::*;

def_class_with_variant!(SeparatorClass, "bg-border shrink-0", vertical: SeparatorOrientation);

def_variant!(
    SeparatorOrientation,
    Horizontal => r#"w-full h-[1px]"#,
    Vertical => r#"h-full w-[1px]"#
);

impl From<bool> for SeparatorOrientation {
    fn from(value: bool) -> Self {
        match value {
            true => SeparatorOrientation::Vertical,
            false => SeparatorOrientation::Horizontal,
        }
    }
}
