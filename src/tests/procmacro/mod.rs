use dioxus::prelude::*;
use myderive::dxcomp;

#[derive(Clone, PartialEq)]
enum Color {
    Primary,
}

// Use 'cargo expand --lib test' to see gererated output by the macro
#[dxcomp]
#[derive(Clone, PartialEq, Props)]
struct StructTest {
    field1: i32,
}

#[dxcomp(color)]
#[derive(Clone, PartialEq, Props)]
struct StructTestColor {
    field1: i32,
}

// Not working because [dxcomp] is made for structs only
// #[dxcomp]
// enum TestEnum {
//     Variant1,
//     Variant2,
// }
