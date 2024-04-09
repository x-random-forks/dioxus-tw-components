// Usage :
// props_no_children!(MyCoolProps {});
//
// Output :
// #[derive(Clone, Component, Props, PartialEq)]
// pub struct MyCoolProps {
//     #[props(default = crate::hooks::use_unique_id())]
//     id: String,
//     #[props(default)]
//     class: String,
// }
macro_rules! props_no_children {
    ($name:ident { $($(#[$attr:meta])? $field:ident : $type:ty),* $(,)? } ) => {
        #[derive(Clone, Component, Props, PartialEq)]
        pub struct $name {
            #[props(default = crate::hooks::use_unique_id())]
            id: String,

            #[props(default)]
            class: String,

            $($(#[$attr])? $field: $type,)*
        }
    };
}

// Usage :
// props!(MyCoolProps {
//     #[props(default = false)]
//     required: bool,
// });
//
// Output :
// #[derive(Clone, Component, Props, PartialEq)]
// pub struct MyCoolProps {
//     #[props(default = crate::hooks::use_unique_id())]
//     id: String,
//     #[props(default)]
//     class: String,
//     children: Element,
//     #[props(default = false)]
//     required: bool,
// }
macro_rules! props {
    ($name:ident { $($(#[$attr:meta])? $field:ident : $type:ty),* $(,)? } ) => {
        #[derive(Clone, Component, Props, PartialEq)]
        pub struct $name {
            #[props(default = crate::hooks::use_unique_id())]
            id: String,

            #[props(default)]
            class: String,

            children: Element,

            $($(#[$attr])? $field: $type,)*
        }
    };
}
