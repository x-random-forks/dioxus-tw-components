macro_rules! class {
    ($name:ident, $class:expr) => {
        #[derive(TwClass, Clone, Copy)]
        #[tw(class = $class)]
        pub struct $name {}
    };
}
