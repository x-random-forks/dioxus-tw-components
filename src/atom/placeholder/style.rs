use tailwind_fuse::*;

def_class_with_variant!(PlaceholderClass, r#"bg-muted"#, radius: PlaceholderRadius, animation: PlaceholderAnimation);

def_variant!(
    PlaceholderRadius,
    Global => r#"rounded-global-radius"#,
    Full => r#"rounded-full"#,
    None => r#"rounded-none"#
);

def_variant!(
    PlaceholderAnimation,
    Pulse => r#"animate-pulse"#,
    None => r#""#
);

impl From<bool> for PlaceholderAnimation {
    fn from(animated: bool) -> Self {
        match animated {
            true => PlaceholderAnimation::Pulse,
            false => PlaceholderAnimation::None,
        }
    }
}
