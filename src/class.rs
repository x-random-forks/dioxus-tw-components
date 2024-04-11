// Usage :
// class!(MyCoolClass, "my-cool-class");
//
// The String Literal corresponds to the class CSS
//
// Output :
// #[derive(TwClass, Clone, Copy)]
// #[tw(class = "my-cool-class")]
// pub struct MyCoolClass {}
macro_rules! def_class_no_variant {
    ($name:ident, $class:expr) => {
        #[derive(TwClass, Clone, Copy)]
        #[tw(class = $class)]
        pub struct $name {}
    };
}

// Usage :
// define_variants_with_classes!(MyCoolClass, "my-cool-class", variant: MyCoolClassVariant, size: MyCoolClassSize);
//
// The String Literal corresponds to the class CSS
//
// Output :
// #[derive(TwClass, Clone, Copy)]
// #[tw(class = "my-cool-class")]
// pub struct MyCoolClass {
//     variant: MyCoolClassVariant,
//     size: MyCoolClassSize,
// }
macro_rules! def_class_with_variant {
    ($name:ident, $class:expr, $($field_name:ident : $field_type:ty),* $(,)?) => {
        #[derive(TwClass, Clone, Copy)]
        #[tw(class = $class)]
        pub struct $name {
            $(
                $field_name: $field_type,
            )*
        }
    };
}

// Usage :
// define_variants_with_classes!( MyCoolEnum, Variant0 => "class0", Variant1 => "class1", Variant2 => "class2" );
//
// The first variant is the default one !
//
// Output :
// #[derive(TwVariant, PartialEq)]
// pub enum MyCoolEnum {
//     #[tw(default, class = "class0")]
//     Variant0,
//     #[tw(class = "class1")]
//     Variant1,
//     #[tw(class = "class2")]
//     Variant2,
// }
macro_rules! def_variant {
    ($name:ident, $default_variant:ident => $default_class:expr, $($variant:ident => $class:expr),* $(,)?) => {
        #[derive(TwVariant, PartialEq)]
        pub enum $name {
            #[tw(default, class = $default_class)]
            $default_variant,
            $(
                #[tw(class = $class)]
                $variant,
            )*
        }
    };
}

// TESTING
macro_rules! impl_colorable {
    ($type:ty, $($variant:ident => $color:expr),* $(,)?) => {
        impl Colorable for $type {
            fn color(&self) -> &'static str {
                match self.color {
                    $(
                        Color::$variant => $color,
                    )*
                }
            }
        }
    };
}

macro_rules! impl_base_class {
    ($type:ty, $base:expr) => {
        impl BaseClass for $type {
            fn base(&self) -> &'static str {
                $base
            }
        }
    };
}
