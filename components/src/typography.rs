use std::fmt::Display;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;
use crate::size::Size;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextColour {
    Colour(Colour),
}

impl ClassName for TextColour {
    fn has_class_name(self) -> bool {
        self != TextColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        match self {
            TextColour::Colour(Colour::Default) => "".to_string(),
            TextColour::Colour(color) => format!("text-{}", color),
        }
    }
}

impl Display for TextColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextContentColour {
    Colour(Colour),
}

impl ClassName for TextContentColour {
    fn has_class_name(self) -> bool {
        self != TextContentColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        match self {
            TextContentColour::Colour(Colour::Default) => "".to_string(),
            TextContentColour::Colour(colour) => {
                format!("text-{}", colour.to_content_colour())
            }
        }
    }
}

impl Display for TextContentColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextSize {
    Size(Size),
}

impl ClassName for TextSize {
    fn has_class_name(self) -> bool {
        self != TextSize::Size(Size::Default)
    }

    fn class_name(self) -> String {
        match self {
            TextSize::Size(Size::Default) => "".to_string(),
            TextSize::Size(size) => format!("text-{}", size),
        }
    }
}

impl Display for TextSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}
