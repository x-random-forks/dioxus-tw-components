// Usage :
// class!(MyCoolClass, "my-cool-class");
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
// Output :
// #[derive(TwClass, Clone, Copy)]
// #[tw(class = "default-class")]
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
// define_variants_with_classes!( MyCoolEnum, DefaultVariant => "default-class", Variant1 => "class1", Variant2 => "class2" );
//
// The first variant is the default one !
//
// Output :
// #[derive(TwVariant, PartialEq)]
// pub enum MyCoolEnum {
//     #[tw(default, class = "default-class")]
//     DefaultVariant,
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
