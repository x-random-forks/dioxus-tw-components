use dioxus::prelude::*;
use dioxus_components_bin::{
    components::{
        atom::button::Button,
        composite::lightswitch::LightSwitchSignal,
        form::{form::Form, input::Input, radiogroup::*, toggle::Toggle},
    },
    types::Side,
};
use std::collections::HashMap;

use crate::website::router::Route;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(LightSwitchSignal("".to_string())));

    let light_switch_context = use_context::<Signal<LightSwitchSignal>>();
    let dark = &light_switch_context.read().0;
    rsx!(
        div { class: "{dark} bg-background text-foreground min-h-screen", Router::<Route> {} }
    )
}

pub fn HomePage() -> Element {
    rsx!(
        p { class: "", "Hello World" }
        TestIntegrationFormBuilder {}
    )
}

fn TestIntegrationFormBuilder() -> Element {
    let mut values = use_signal(HashMap::new);
    let mut submitted_values = use_signal(HashMap::new);

    let form = use_form().unwrap();
    let form = form.iter().unwrap();
    let mut vecform: Vec<Element> = Vec::new();

    let _form = form.for_each(|field| {
        let label = rsx!( label { "{field.label}" } );

        let forminput = match &field.content {
            FieldType::Text(_) => {
                let input = rsx!(
                    Input {
                        r#type: "text",
                        name: "login",
                        oninput: move |event: FormEvent| {
                            values.set(event.values());
                        }
                    }
                );
                input
            }
            FieldType::Radio(radio) => {
                let radiogroup = rsx!(
                    RadioGroup { name: "rcnp",
                        for item in &radio.variants {
                            RadioItem { side: Side::Right, value: item, name: "rcnp", {item.clone()} }
                        }
                    }
                );
                radiogroup
            }
            FieldType::Toggle(toggle) => {
                let toggle = rsx!( Toggle { checked: toggle.default } );
                toggle
            }
            _ => None,
        };
        vecform.push(rsx!(
            div { {label}, {forminput} }
        ));
    });

    let rendered_form = vecform.iter().map(|elem| rsx!({ elem }));

    let onsubmit = move |event: FormEvent| {
        submitted_values.set(event.values());
    };

    let oninput = move |event: FormEvent| {
        values.set(event.values());
    };

    rsx!(
        div {
            div {
                Form { class: "border border-black", onsubmit: onsubmit, oninput: oninput,

                    {rendered_form},
                    Button { r#type: "submit", value: "Submit", "Submit" }
                }
            }
            div { "{submitted_values:#?}" }
        }
    )
}

use form_builder::prelude::*;

fn use_form() -> Result<Form, form_builder::error::FormError> {
    let form = Form::new("Past And Future")
        .push(FormField::new("Login:", FieldType::Text(TextField::new())))?
        .push(FormField::new(
            "Wanted RNCP",
            FieldType::Radio(RadioField::new(vec!["None", "RNCP6", "RNCP7"])?),
        ))?
        .push(FormField::new(
            "Accept terms and conditions ",
            FieldType::Toggle(ToggleField::new()),
        ))?;
    // let mut answer = form.new_answer();
    Ok(form)
}
