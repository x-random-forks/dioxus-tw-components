#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_components_bin::atom::button::*;
use dioxus_components_bin::atom::checkbox::*;
use dioxus_components_bin::atom::formrange::*;
use dioxus_components_bin::atom::icon::style::IconSvg;
use dioxus_components_bin::atom::icon::*;
use dioxus_components_bin::atom::input::*;
use dioxus_components_bin::atom::label::*;
use dioxus_components_bin::atom::separator::*;
use dioxus_components_bin::atom::textarea::*;
use dioxus_components_bin::atom::toggle::*;
use dioxus_components_bin::composite::formlist::*;
use dioxus_components_bin::composite::lightswitch::*;
use dioxus_components_bin::composite::radiogroup::*;
use dioxus_components_bin::composite::select::*;

use website::app::App;

mod website;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::WARN)
            .build(),
    );
    launch(App);
}

// Atoms
// fn TestButton() -> Element {
//     let keyboard_closure = move |event: FormEvent| log::debug!("{}", event.value());
//     rsx!(
//         div { class: "",
//             Button { color: Unset, "Unset" }
//             Button { color: Primary, "Primary" }
//             Button { color: Secondary, "Secondary" }
//             Button { color: Accent, "Accent" }
//             Button { variant: Outline(Primary), "Outline" }
//             Button { variant: Outline(Secondary), "Outline Secondary" }
//             Button { variant: Outline(Accent), "Outline Accent" }
//             Button { variant: Ghost(Primary), "Ghost" }
//             Button { variant: Ghost(Secondary), "Ghost Secondary" }
//             Button { variant: Ghost(Accent), "Ghost Accent" }
//             Button { size: Xs, variant: Ghost(Primary), "Ghost Primary Xs" }
//         }
//         div { class: "",
//             Button { size: Sm, "Sm" }
//             Button { "Default" }
//             Button { size: Lg, "Lg" }
//             Button { size: Xl, "Xl" }
//         }
//         div { class: "", TextArea { oninput: keyboard_closure } }
//     )
// }

fn TestCheckbox() -> Element {
    rsx!(div {
        class: "flex gap-2"
    })
}

fn TestFormRange() -> Element {
    rsx!(
        div { class: "flex",
            div { class: "",
                FormRange { name: "range", min: 0, max: 100, step: 1 }
                FormRange { name: "range", min: 0, max: 100, step: 1, disabled: true }
            }
        }
    )
}

fn TestIcon() -> Element {
    rsx!(
        div { class: "flex gap-4",
            // TODO
            // div { class: "size-8", Icon { svg: IconSvg::CircleInnerCircle, color: Default } }
            div { class: "size-8", Icon { svg: IconSvg::CircleInnerCircle, color: Primary } }
            div { class: "size-8", Icon { svg: IconSvg::CircleInnerCircle, color: Secondary } }
            div { class: "size-8", Icon { svg: IconSvg::CircleInnerCircle, color: Accent } }
            div { class: "size-8", Icon { svg: IconSvg::HollowCircle, color: Primary } }
            div { class: "size-8", Icon { svg: IconSvg::HollowCircle, color: Secondary } }
            div { class: "size-8", Icon { svg: IconSvg::HollowCircle, color: Accent } }
        }
    )
}

fn TestInput() -> Element {
    rsx!(

        div { class: "flex gap-4",
            div { class: "",
                div { class: "flex", Input { r#type: "text", name: "text1", id: "text1", placeholder: "Text" } }
                Input { r#type: "text", name: "text", placeholder: "Text", disabled: true }
            }
            div { class: "",
                Input { r#type: "email", name: "email", placeholder: "Email" }
                Input { r#type: "email", name: "email", placeholder: "Email", disabled: true }
            }
            div { class: "",
                Input { r#type: "number", name: "number", placeholder: "Number" }
                Input { r#type: "number", name: "number", placeholder: "Number", disabled: true }
            }
            div { class: "",
                Input { r#type: "date", name: "date", placeholder: "Date" }
                Input { r#type: "date", name: "date", placeholder: "Date", disabled: true }
            }
            div { class: "",
                Input { r#type: "file", name: "date", placeholder: "File" }
                Input { r#type: "file", name: "date", placeholder: "File", disabled: true }
            }
        }
    )
}

fn TestTextArea() -> Element {
    rsx!(
        div { class: "flex gap-4",
            div { class: "",
                TextArea { name: "textarea", placeholder: "TextArea" }
                TextArea { name: "textarea", placeholder: "TextArea", disabled: true }
            }
        }
    )
}

fn TestSeparator() -> Element {
    rsx!(
        div {
            div { class: "inline-block",
                p { "Dioxus comp lib" }
                Separator { class: "my-4" }
                div { class: "flex h-5 items-center space-x-4 text-sm",
                    div { "AAAAA" }
                    Separator { vertical: true }
                    div { "BBBBBBBBBBBBBBBBBBBBBBB" }
                    Separator { vertical: true }
                    div { "ZZ" }
                }
            }
        }
    )
}

fn TestToggle() -> Element {
    rsx!(
        // div { class: "flex gap-4",
        //     div { class: "",
        //         Toggle { name: "toggle", value: "toggle", checked: true, color: Primary, "Primary" }
        //         Toggle { name: "toggle", value: "toggle", checked: true, color: Secondary, "Secondary" }
        //         Toggle { name: "toggle", value: "toggle", checked: true, color: Accent, "Accent" }
        //         Toggle {
        //             name: "toggle",
        //             value: "toggle",
        //             checked: true,
        //             color: Primary,
        //             disabled: true,
        //             "Disabled"
        //         }
        //         Toggle { name: "toggle", value: "toggle", checked: true, color: Primary, size: Sm, "Sm" }
        //         Toggle { name: "toggle", value: "toggle", checked: true, color: Primary, size: Lg, "Lg" }
        //     }
        // }
    )
}

fn TestForm() -> Element {
    let mut values = use_signal(HashMap::new);
    rsx!(
        div {
            "Test"
            form {
                class: "border border-black w-96",
                method: "POST",
                id: "option-form",
                onsubmit: move |event| {
                    log::debug!("Form values {:#?}", values());
                    log::debug!("Valid :{}", event.valid());
                    values.set(event.values());
                },
                // oninput: move |event| {
                //     values.set(event.values());
                // },
                RadioGroup { name: "gender", default_value: "male",
                    Label { r#for: "gender", "Choose birth gender" }
                    RadioItem { value: "male", name: "gender", required: true, "Male" }
                    RadioItem { value: "female", name: "gender", "Female" }
                    RadioItem { value: "other", name: "gender", "Other" }
                    RadioItem { value: "disabled", name: "gender", disabled: true, "Disabled" }
                }
                div { class: "flex flex-col",
                    Label { r#for: "username", "Your username" }
                    Input { r#type: "text", name: "username", placeholder: "username" }
                }
                div {
                    Label { r#for: "email", "Your email" }
                    Input { r#type: "email", name: "email", placeholder: "email" }
                }
                div {
                    Label { r#for: "age", "Your age" }
                    Input { r#type: "number", name: "age", placeholder: "age", min: 18, max: 100 }
                }
                div {
                    Label { r#for: "date", "Your Piscine date" }
                    Input { r#type: "date", name: "date", placeholder: "Piscine date" }
                }
                div {
                    Label { r#for: "message", "Send us a message" }
                    TextArea { name: "message", placeholder: "Your message..." }
                }
                div {
                    Label { r#for: "animal", "Select an animal" }
                    SelectGroup { name: "animal",
                        SelectPlaceholder { "Select an animal" }
                        SelectLabel { label: "Domestic",
                            SelectItem { value: "dog", "Dog" }
                            SelectItem { value: "cat", "Cat" }
                            SelectItem { value: "hamster", "Hamster" }
                            SelectItem { value: "none", disabled: true, "None" }
                        }
                        SelectLabel { label: "Wild", disabled: true,
                            SelectItem { value: "lion", "Lion" }
                            SelectItem { value: "tiger", "Tiger" }
                            SelectItem { value: "bear", "Bear" }
                        }
                    }
                }
                div {
                    Label { r#for: "activities", "Select your fav activities" }
                    Checkbox { name: "activities", value: "reading", "Reading" }
                    Checkbox { name: "activities", value: "coding", checked: true, "Coding" }
                    Checkbox { name: "activities", value: "writing", "Writing" }
                    Checkbox { name: "activities", value: "swimming", "Swimming" }
                    Checkbox { name: "activities", value: "football", "Football" }
                    Checkbox { name: "activities", value: "none", disabled: true,
                        Label { "None" }
                    }
                }
                div {
                    Label { r#for: "rate", "Rate us" }
                    FormRange { name: "rate", min: 0, max: 100, step: 1 }
                }
                div {
                    Toggle { name: "newsletter", value: "newsletter", checked: true,
                        Label { r#for: "newsletter", "Subscribe to newsletter" }
                    }
                }
                div {
                //     Toggle { name: "toggle1", value: "toggle1", checked: true, color: Secondary,
                //         Label { "toggle1" }
                //     }
                // }
                // div {
                //     Toggle {
                //         name: "toggle2",
                //         value: "toggle2",
                //         checked: true,
                //         color: Accent,
                //         size: Sm,
                //         Label { "toggle2" }
                //     }
                }
                div {
                    // Toggle { name: "toggle3", value: "toggle3", disabled: true, size: Lg,
                    //     Label { "toggle3" }
                    // }
                }
                div {
                    Toggle { name: "cookie", value: "cookie", checked: false,
                        Label { "Use our cookie" }
                    }
                }
                div {
                    Checkbox { name: "terms", value: "yes", required: false,
                        Label { "Accept terms and conditions" }
                    }
                }
                div {
                    // Button { variant: Ghost(Primary), size: Sm, r#type: "submit", "Submit" }
                }
            }
        }
        div { "Values: {values:#?}" }
    )
}

// Composites

fn TestListForm() -> Element {
    let mut values = use_signal(HashMap::new);
    // let input1 = rsx!(
    //     SelectGroup { name: "animal",
    //         SelectPlaceholder { "Select an animal" }
    //         SelectLabel { label: "Domestic",
    //             SelectItem { value: "dog", "Dog" }
    //             SelectItem { value: "cat", "Cat" }
    //             SelectItem { value: "hamster", "Hamster" }
    //             SelectItem { value: "none", disabled: true, "None" }
    //         }
    //         SelectLabel { label: "Wild", disabled: true,
    //             SelectItem { value: "lion", "Lion" }
    //             SelectItem { value: "tiger", "Tiger" }
    //             SelectItem { value: "bear", "Bear" }
    //         }
    //     }
    //     SelectGroup { name: "number-animal",
    //         SelectPlaceholder { "Select number of animals" }
    //         SelectLabel { label: "1-10",
    //             SelectItem { value: "1", "1" }
    //             SelectItem { value: "2", "2" }
    //             SelectItem { value: "3", "3" }
    //             SelectItem { value: "4", "4" }
    //             SelectItem { value: "5", "5" }
    //             SelectItem { value: "6", "6" }
    //             SelectItem { value: "7", "7" }
    //             SelectItem { value: "8", "8" }
    //             SelectItem { value: "9", "9" }
    //             SelectItem { value: "10", "10" }
    //         }
    //     }
    //     Label { r#for: "name-animal", "Type animal name" }
    //     Input { r#type: Text, name: "name-animal", placeholder: "Animal name" }
    // );

    // TODO Should do this in a Macro
    let mut group = Vec::<Element>::new();
    for i in 0..5 {
        group.push(rsx!(
            SelectGroup { name: "animal-{i}",
                SelectPlaceholder { "Select an animal" }
                SelectLabel { label: "Domestic",
                    SelectItem { value: "dog", "Dog" }
                    SelectItem { value: "cat", "Cat" }
                    SelectItem { value: "hamster", "Hamster" }
                    SelectItem { value: "none", disabled: true, "None" }
                }
                SelectLabel { label: "Wild", disabled: true,
                    SelectItem { value: "lion", "Lion" }
                    SelectItem { value: "tiger", "Tiger" }
                    SelectItem { value: "bear", "Bear" }
                }
            }
            SelectGroup { name: "number-animal-{i}",
                SelectPlaceholder { "Select number of animals" }
                SelectLabel { label: "1-10",
                    SelectItem { value: "1", "1" }
                    SelectItem { value: "2", "2" }
                    SelectItem { value: "3", "3" }
                    SelectItem { value: "4", "4" }
                    SelectItem { value: "5", "5" }
                    SelectItem { value: "6", "6" }
                    SelectItem { value: "7", "7" }
                    SelectItem { value: "8", "8" }
                    SelectItem { value: "9", "9" }
                    SelectItem { value: "10", "10" }
                }
            }
            Label { r#for: "name-animal-{i}", "Type animal name" }
            Input { r#type: "text", name: "name-animal-{i}", placeholder: "Animal name" }
        ));
    }

    rsx!(
        div {
            "TestListForm"
            div { class: "border border-black w-96",
                form {
                    // method: "POST",
                    id: "id-group",
                    onsubmit: move |event| {
                        log::debug!("Form values {:#?}", values());
                        log::debug!("Valid :{}", event.valid());
                        values.set(event.values());
                    },
                    FormList { group_vec: group }
                }
            }
        }
    )
}

fn TestLightSwitch() -> Element {
    rsx!(
        div { class: "flex gap-4",
            div { class: "", LightSwitch {} }
        }
    )
}

fn TestRadioGroup() -> Element {
    rsx!(
        RadioGroup { name: "gender", default_value: "male",
            Label { r#for: "gender", "Choose birth gender" }
            RadioItem { value: "male", name: "gender", required: true, "Male" }
            RadioItem { value: "female", name: "gender", "Female" }
            RadioItem { value: "other", name: "gender", "Other" }
            RadioItem { value: "disabled", name: "gender", disabled: true, "Disabled" }
        }
    )
}

fn TestSelect() -> Element {
    rsx!(
        div { class: "flex w-96 gap-4",
            SelectGroup { name: "animal",
                SelectPlaceholder { "Select an animal" }
                SelectLabel { label: "Domestic",
                    SelectItem { value: "dog", "Dog" }
                    SelectItem { value: "cat", "Cat" }
                    SelectItem { value: "hamster", "Hamster" }
                    SelectItem { value: "none", disabled: true, "None" }
                }
                SelectLabel { label: "Wild", disabled: true,
                    SelectItem { value: "lion", "Lion" }
                    SelectItem { value: "tiger", "Tiger" }
                    SelectItem { value: "bear", "Bear" }
                }
            }
        }
    )
}

fn TestDragDrop() -> Element {
    let class = "border-4 border-black bg-input w-96 cursor-move box";
    let _ = use_resource(move || async move {
        let mut eval = eval(
            r#"
            function handleDragStart(e) {
                this.style.opacity = '0.4';
              
                dragSrcEl = this;
              
                e.dataTransfer.effectAllowed = 'move';
                e.dataTransfer.setData('text/html', this.innerHTML);
              }
              
                function handleDragEnd(e) {
                  this.style.opacity = '1';
                  items.forEach(function (item) {
                    item.classList.remove('over');
                  });
                }

                function handleDragOver(e) {
                    e.preventDefault();
                    return false;
                  }

                  function handleDragEnter(e) {
                    this.classList.add('over');
                  }
                
                  function handleDragLeave(e) {
                    this.classList.remove('over');
                  }

                  function handleDrop(e) {
                    e.stopPropagation();
                  
                    if (dragSrcEl !== this) {
                      dragSrcEl.innerHTML = this.innerHTML;
                      this.innerHTML = e.dataTransfer.getData('text/html');
                    }
                  
                    return false;
                  }
              
                let items = document.querySelectorAll('.box');
                items.forEach(function(item) {    
                    item.addEventListener('dragstart', handleDragStart);
                    item.addEventListener('dragover', handleDragOver);
                    item.addEventListener('dragenter', handleDragEnter);
                    item.addEventListener('dragleave', handleDragLeave);
                    item.addEventListener('dragend', handleDragEnd);
                    item.addEventListener('drop', handleDrop);
                });
        "#,
        );
    });

    rsx!(
        div { class: "grid grid-cols-3 gap-2",
            div { draggable: true, class: "{class}", "VALENTIN" }
            div { draggable: true, class: "{class}", "JULES" }
            div { draggable: true, class: "{class}", "ANTHONY" }
            div { draggable: true, class: "{class}", "JULIEN" }
            div { draggable: true, class: "{class}", "JEANNE" }
            div { draggable: true, class: "{class}", "MATHIEU" }
        }
    )
}
