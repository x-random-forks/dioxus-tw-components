#[macro_export]
macro_rules! class {
    ($($style:expr),*) => {
        [$($style.to_string()),*].join(" ")
    };
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum BaseClass<T = ()> {
    Default,
    _Phantom(std::marker::PhantomData<T>),
}

#[derive(Default, PartialEq, Clone, Debug, Copy)]
pub enum Color<T = ()> {
    Unset,
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    _Phantom(std::marker::PhantomData<T>),
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Color::Default => "bg-background text-foreground border-foreground",
            Color::Primary => "bg-primary text-primary-foreground border-primary",
            Color::Secondary => "bg-secondary text-secondary-foreground border-secondary",
            Color::Accent => "bg-accent text-accent-foreground border-accent",
            _ => "",
        };
        write!(f, "{}", size)
    }
}

#[derive(Default, PartialEq, Clone, Debug, Copy)]
pub enum Size<T = ()> {
    #[default]
    Md,
    Lg,
    Sm,
    Xl,
    Xs,
    _Phantom(std::marker::PhantomData<T>),
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::Md => "text-base",
            Size::Lg => "text-lg",
            Size::Sm => "text-sm",
            Size::Xl => "text-xl",
            Size::Xs => "text-xs",
            _ => panic!(),
        };
        write!(f, "{}", size)
    }
}

// List all of possible variants for every component,
// Not every component need a variant and not every component has the same variant
#[derive(Default, PartialEq, Clone, Debug, Copy)]
pub enum Variant<T = ()> {
    #[default]
    Default,
    Outline(Color),
    Ghost(Color),
    _Phantom(std::marker::PhantomData<T>),
}
