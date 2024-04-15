// // impl BaseClass for TestTextProps {
// //     fn base(&self) -> &'static str {
// //         "font-bold"
// //     }
// // }
// impl_base_class! {TestTextProps, "font-bold"}

// // impl Colorable for TestTextProps {
// //     fn color(&self) -> &'static str {
// //         match self.color {
// //             Color::Default => "text-foreground",
// //             Color::Primary => "text-primary",
// //             Color::Secondary => "text-secondary",
// //             Color::Destructive => "text-destructive",
// //             Color::Accent => "text-accent",
// //             Color::Muted => "text-muted",
// //         }
// //     }
// // }
// impl_colorable! {
//     TestTextProps,
//     Default => "text-foreground",
//     Primary => "text-primary",
//     Secondary => "text-secondary",
//     Destructive => "text-destructive",
//     Success => "text-success",
//     Accent => "text-accent",
//     Muted => "text-muted"
// }

// #[derive(Default, Clone, Copy, PartialEq)]
// pub enum TestTextVariant {
//     #[default]
//     Default,
//     Italic,
//     Bold,
//     Underline,
// }

// impl Variation for TestTextProps {
//     fn variant(&self) -> &'static str {
//         match self.variant {
//             TestTextVariant::Default => "",
//             TestTextVariant::Italic => "italic",
//             TestTextVariant::Bold => "font-black",
//             TestTextVariant::Underline => "underline",
//         }
//     }
// }

// impl_base_class! {TestBoxProps, "border border-foreground w-10 h-10"}

// impl_colorable! {
//     TestBoxProps,
//     Default => "bg-background",
//     Primary => "bg-primary",
//     Secondary => "bg-secondary",
//     Destructive => "bg-destructive",
//     Success => "bg-success",
//     Accent => "bg-accent",
//     Muted => "bg-muted"
// }

// #[derive(Default, Clone, Copy, PartialEq)]
// pub enum TestBoxVariant {
//     #[default]
//     Default,
//     Rounded,
//     Circle,
// }

// impl Variation for TestBoxProps {
//     fn variant(&self) -> &'static str {
//         match self.variant {
//             TestBoxVariant::Default => "",
//             TestBoxVariant::Rounded => "rounded-xl",
//             TestBoxVariant::Circle => "rounded-full",
//         }
//     }
// }
