use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HorizontalAlign {
    Start,
    Center,
    End,
}

impl Display for HorizontalAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HorizontalAlign::Start => write!(f, "start"),
            HorizontalAlign::Center => write!(f, "center"),
            HorizontalAlign::End => write!(f, "end"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl Display for VerticalAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerticalAlign::Top => write!(f, "top"),
            VerticalAlign::Middle => write!(f, "middle"),
            VerticalAlign::Bottom => write!(f, "end"),
        }
    }
}
