use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Orientation {
    Default,
    Horizontal,
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Default => write!(f, ""),
            Orientation::Horizontal => write!(f, "horizontal"),
            Orientation::Vertical => write!(f, "vertical"),
        }
    }
}
