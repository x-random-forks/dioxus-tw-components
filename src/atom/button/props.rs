use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

props! {
    ButtonProps {
        #[props(extends = button)]
        attributes: Vec<Attribute>,

        #[props(optional)]
        onclick: Option<EventHandler<MouseEvent>>,

        #[props(default)]
        variant: super::ButtonVariant,
        #[props(default)]
        color: super::ButtonColor,
        #[props(default)]
        size: super::ButtonSize,
    }
}

impl Component for ButtonProps {
    fn view(self) -> Element {
        let class = super::ButtonClass::builder()
            .color(self.color)
            .size(self.size)
            .variant(self.variant)
            .with_class(self.class);

        // TO FIX : tailwind fuse not working as expected, temporary fix below
        // In the end we should be able to just move this in the ButtonVariant in style.rs
        let class = tw_merge!(
            class,
            match self.variant {
                super::ButtonVariant::Default => "",
                super::ButtonVariant::Outline => match self.color {
                    super::ButtonColor::Default => "border border-foreground bg-background hover:text-accent-foreground",
                    super::ButtonColor::Primary => "border border-primary text-primary bg-background hover:text-primary-foreground",
                    super::ButtonColor::Secondary =>
                        "border border-secondary text-secondary bg-background hover:text-secondary-foreground",
                    super::ButtonColor::Destructive =>
                        "border border-destructive text-destructive bg-background hover:text-destructive-foreground",
                },
                super::ButtonVariant::Ghost => match self.color {
                    super::ButtonColor::Default => "border-none bg-background text-foreground hover:bg-accent/80 hover:text-accent-foreground",
                    super::ButtonColor::Primary => "border-none bg-background text-primary hover:bg-primary/80 hover:text-primary-foreground",
                    super::ButtonColor::Secondary => "border-none bg-background text-secondary hover:bg-secondary/80 hover:text-secondary-foreground",
                    super::ButtonColor::Destructive => "border-none bg-background text-destructive hover:bg-destructive/80 hover:text-destructive-foreground"
                },
            }
        );

        let onclick = move |event| {
            if let Some(oc) = &self.onclick {
                oc.call(event)
            }
        };

        rsx!(
            button {
                ..self.attributes,
                class: "{class}",
                id: self.id,
                onclick: onclick,
                {self.children}
            }
        )
    }
}
