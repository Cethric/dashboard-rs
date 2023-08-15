use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Size {
    Default,
    Large,
    Medium,
    Small,
    ExtraSmall,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Default => write!(f, ""),
            Size::Large => write!(f, "lg"),
            Size::Medium => write!(f, "md"),
            Size::Small => write!(f, "sm"),
            Size::ExtraSmall => write!(f, "xs"),
        }
    }
}
