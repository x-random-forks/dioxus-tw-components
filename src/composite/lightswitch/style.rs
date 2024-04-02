use super::LightSwitchProps;
use crate::styling::Size;

impl std::fmt::Display for Size<LightSwitchProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::Lg => "size-8",
            Size::Sm => "size-4",
            Size::Xl => "size-10",
            Size::Xs => "size-2",
            Size::Md | _ => "size-6",
        };
        write!(f, "{}", size)
    }
}
