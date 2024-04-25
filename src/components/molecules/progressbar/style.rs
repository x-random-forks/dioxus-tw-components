use super::props::*;
use crate::attributes::*;

impl BaseClass for ProgressTrackProps {
    fn base(&self) -> &'static str {
        "w-full rounded-global-radius"
    }
}

impl Colorable for ProgressTrackProps {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Default => "bg-background",
            Color::Primary => "bg-primary",
            Color::Secondary => "bg-secondary",
            Color::Destructive => "bg-destructive",
            Color::Success => "bg-success",
            Color::Accent => "bg-accent",
            Color::Muted => "bg-muted",
        }
    }
}

impl Sizable for ProgressTrackProps {
    fn size(&self) -> &'static str {
        match self.size {
            Size::Xs | Size::Sm => "h-2 d-text-extrasmall",
            Size::Md => "h-4 d-text-small",
            Size::Lg => "h-6 d-text-medium",
            Size::Xl => "h-8 d-text-large",
        }
    }
}

impl BaseClass for ProgressBarProps {
    fn base(&self) -> &'static str {
        "h-full rounded-global-radius flex items-center justify-center"
    }
}

impl Colorable for ProgressBarProps {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Default => "bg-background [&>*]:text-foreground",
            Color::Primary => "bg-primary [&>*]:text-primary-foreground",
            Color::Secondary => "bg-secondary [&>*]:text-secondary-foreground",
            Color::Destructive => "bg-destructive [&>*]:text-destructive-foreground",
            _ => "This Color is not implemented for ProgressBar",
        }
    }
}

impl BaseClass for ProgressLabelProps {
    fn base(&self) -> &'static str {
        ""
    }
}
