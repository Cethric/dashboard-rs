use std::fmt::Display;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Default,
    Primary,
    PrimaryFocus,
    PrimaryContent,
    Secondary,
    SecondaryFocus,
    SecondaryContent,
    Accent,
    AccentFocus,
    AccentContent,
    Neutral,
    NeutralFocus,
    NeutralContent,
    Base100,
    Base200,
    Base300,
    BaseContent,
    Info,
    InfoContent,
    Success,
    SuccessContent,
    Warning,
    WarningContent,
    Error,
    ErrorContent,
    // Ghost,
    // Link,
}

impl Colour {
    pub(crate) fn to_content_colour(&self) -> Colour {
        match self {
            Colour::Default => Colour::Default,
            Colour::Primary => Colour::PrimaryContent,
            Colour::PrimaryFocus => Colour::PrimaryContent,
            Colour::PrimaryContent => Colour::PrimaryContent,
            Colour::Secondary => Colour::SecondaryContent,
            Colour::SecondaryFocus => Colour::SecondaryContent,
            Colour::SecondaryContent => Colour::SecondaryContent,
            Colour::Accent => Colour::AccentContent,
            Colour::AccentFocus => Colour::AccentContent,
            Colour::AccentContent => Colour::AccentContent,
            Colour::Neutral => Colour::NeutralContent,
            Colour::NeutralFocus => Colour::NeutralContent,
            Colour::NeutralContent => Colour::NeutralContent,
            Colour::Base100 => Colour::BaseContent,
            Colour::Base200 => Colour::BaseContent,
            Colour::Base300 => Colour::BaseContent,
            Colour::BaseContent => Colour::BaseContent,
            Colour::Info => Colour::InfoContent,
            Colour::InfoContent => Colour::InfoContent,
            Colour::Success => Colour::SuccessContent,
            Colour::SuccessContent => Colour::SuccessContent,
            Colour::Warning => Colour::WarningContent,
            Colour::WarningContent => Colour::WarningContent,
            Colour::Error => Colour::ErrorContent,
            Colour::ErrorContent => Colour::ErrorContent,
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Default => write!(f, ""),
            Colour::Primary => write!(f, "primary"),
            Colour::PrimaryFocus => write!(f, "primary-focus"),
            Colour::PrimaryContent => write!(f, "primary-content"),
            Colour::Secondary => write!(f, "secondary"),
            Colour::SecondaryFocus => write!(f, "secondary-focus"),
            Colour::SecondaryContent => write!(f, "secondary-content"),
            Colour::Accent => write!(f, "accent"),
            Colour::AccentFocus => write!(f, "accent-focus"),
            Colour::AccentContent => write!(f, "accent-content"),
            Colour::Neutral => write!(f, "neutral"),
            Colour::NeutralFocus => write!(f, "neutral-focus"),
            Colour::NeutralContent => write!(f, "neutral-content"),
            Colour::Base100 => write!(f, "base-100"),
            Colour::Base200 => write!(f, "base-200"),
            Colour::Base300 => write!(f, "base-300"),
            Colour::BaseContent => write!(f, "base-content"),
            Colour::Info => write!(f, "info"),
            Colour::InfoContent => write!(f, "info-content"),
            Colour::Success => write!(f, "success"),
            Colour::SuccessContent => write!(f, "success-content"),
            Colour::Warning => write!(f, "warning"),
            Colour::WarningContent => write!(f, "warning-content"),
            Colour::Error => write!(f, "error"),
            Colour::ErrorContent => write!(f, "error-content"),
            // Colour::Ghost => write!(f, ""),
            // Colour::Link => write!(f, ""),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BorderColourBase<T> {
    Colour(T),
}

pub type BorderColour = BorderColourBase<Colour>;

impl ClassName for BorderColour {
    fn has_class_name(self) -> bool {
        self != BorderColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            BorderColour::Colour(Colour::Default) => "".to_string(),
            BorderColour::Colour(color) => format!("border-{}", color),
            // BorderColour::Colour(Colour::Ghost) => {}
            // BorderColour::Colour(Colour::Link) => {}
        })
    }
}

impl Display for BorderColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BackgroundColourBase<T> {
    Colour(T),
}

pub type BackgroundColour = BackgroundColourBase<Colour>;

impl ClassName for BackgroundColour {
    fn has_class_name(self) -> bool {
        self != BackgroundColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            BackgroundColour::Colour(Colour::Default) => "".to_string(),
            BackgroundColour::Colour(color) => format!("bg-{}", color),
        })
    }
}

impl Display for BackgroundColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}
