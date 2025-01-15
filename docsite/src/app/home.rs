use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx!(
        section { class: "w-full mx-auto flex flex-col justify-between items-center min-h-[760px] flex-1 max-h-[960px] px-4",
            div { class: "flex w-full max-w-screen-xl flex-col text-center md:min-h-[520px] min-h-[760px] h-[calc(100vh-4rem)] gap-2 justify-evenly",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
                    div { class: "text-center lg:text-left lg:flex-1",
                        div { class: "text-[2.5em] md:text-[3.5em] font-semibold text-ghdarkmetal font-sans leading-snug text-balance",
                            span { "Dioxus Lib Comp" }
                        }
                        h3 { class: "text-[1.25em] font-light text-ghdarkmetal max-w-screen-sm md:max-w-screen-md md:text-left text-center flex flex-col",
                            span { class: "max-w-screen-md leading-loose",
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
                            }
                        }
                    }
                    div { class: "lg:pb-12 h-screen max-h-40 lg:max-h-80 my-8",
                        "Identifying as a cool logo"
                    }
                }
            }
        }
    )
}
