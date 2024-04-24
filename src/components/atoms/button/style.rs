use super::props::*;
use crate::attributes::*;

impl BaseClass for ButtonProps {
    fn base(&self) -> &'static str {
        "text-center font-medium rounded-global-radius shadow-global-shadow disabled:opacity-50 disabled:cursor-not-allowed"
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Ghost,
}

impl Variation for ButtonProps {
    fn variant(&self) -> &'static str {
        match self.variant {
            ButtonVariant::Default => match self.color {
                Color::Default => "bg-background text-foreground hover:bg-foreground/20 active:bg-foreground/30 active:shadow",
                Color::Primary => "bg-primary text-primary-foreground border-primary hover:bg-primary/90 active:bg-primary/80 active:shadow",
                Color::Secondary => "bg-secondary text-secondary-foreground border-secondary hover:bg-secondary/90 active:bg-secondary/80 active:shadow",
                Color::Destructive => "bg-destructive text-destructive-foreground border-destructive hover:bg-destructive/90 active:bg-destructive/80 active:shadow",
                Color::Success => "bg-success text-success-foreground border-success hover:bg-success/90 active:bg-success/80 active:shadow",
                Color::Accent => "bg-accent text-accent-foreground border-accent hover:bg-accent/90 active:bg-accent/80 active:shadow",
                Color::Muted => "bg-muted text-muted-foreground border-muted hover:bg-muted/90 active:bg-muted/80 active:shadow",
            },
            ButtonVariant::Outline => match self.color {
                Color::Default => {
                    "border bg-transparent border-foreground text-foreground hover:bg-foreground/40"
                }
                Color::Primary => "border bg-transparent border-primary text-primary hover:bg-primary/90 hover:text-primary-foreground",
                Color::Secondary => "border bg-transparent border-secondary text-secondary hover:bg-secondary/90 hover:text-secondary-foreground",
                Color::Destructive => "border bg-transparent border-destructive text-destructive hover:bg-destructive/90 hover:text-destructive-foreground",
                Color::Success => "border bg-transparent border-success text-success hover:bg-success/90 hover:text-success-foreground",
                Color::Accent => "border bg-transparent border-accent text-accent hover:bg-accent/90 hover:text-accent-foreground",
                Color::Muted => "border bg-transparent border-muted text-muted hover:bg-muted/90 hover:text-muted-foreground",
            },
            ButtonVariant::Ghost => match self.color {
                Color::Default => {
                    "bg-transparent border-foreground text-foreground hover:bg-foreground/40"
                }
                Color::Primary => "bg-transparent border-primary text-primary hover:bg-primary/90 hover:text-primary-foreground",
                Color::Secondary => "bg-transparent border-secondary text-secondary hover:bg-secondary/90 hover:text-secondary-foreground",
                Color::Destructive => "bg-transparent border-destructive text-destructive hover:bg-destructive/90 hover:text-destructive-foreground",
                Color::Success => "bg-transparent border-success text-success hover:bg-success/90 hover:text-success-foreground",
                Color::Accent => "bg-transparent border-accent text-accent hover:bg-accent/90 hover:text-accent-foreground",
                Color::Muted => "bg-transparent border-muted text-muted hover:bg-muted/90 hover:text-muted-foreground",
            },
        }
    }
}

// Handled in variant
impl Colorable for ButtonProps {
    fn color(&self) -> &'static str {
        ""
    }
}

impl Sizable for ButtonProps {
    fn size(&self) -> &'static str {
        match self.size {
            Size::Xs => "px-extrasmall py-extrasmall d-text-extrasmall",
            Size::Sm => "px-small py-small d-text-small",
            Size::Md => "px-small py-small d-text-small",
            Size::Lg => "px-medium py-medium d-text-medium",
            Size::Xl => "px-large py-large d-text-large",
        }
    }
}

impl Animatable for ButtonProps {
    fn animation(&self) -> &'static str {
        match self.animation {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => "transition-colors duration-150",
            Animation::Custom(animation) => animation,
        }
    }
}
