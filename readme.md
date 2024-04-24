This is an internal lib of components.

Basic structure, going straight to the point.

Initial thoughts were to have Style and Theme types to hotswap them on need, but:
- Theme can be changed at runtime simply by replacing CSS
- Style won't be changed at runtime. Compiling for different styles in the same place is just making the whole code heavier.
Meaning if several styles were to be added, it'd simply be done via git branches.

Still implemented a very basic Derive macro because:
- declaring `#[component]` functions directly could be an issue for building documentation and a storybook. So we'll need to stick with props struct.
- We could alternatively just define manually a `pub fn MyComponent(props: MyComponentProps) -> Element {...}`.

Note: right now there's a main + lib and I get a wasm-bindgen conflict but it may just be a local issue. If anyone encounters it, remove the main and integrate the lib in the actual dioxus repo (either manually for now or using git: https://rpadovani.com/private-rust-crates (not tested))



Usage : 

- to compile tailwind `npx tailwindcss -c ./css/tailwind.config.js -i ./css/input.css -o ./public/tailwind.css --watch`
- to serve the lib `dx serve` (you need dioxus CLI `cargo install dioxus-cli@0.5`)