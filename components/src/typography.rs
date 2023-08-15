use std::fmt::Display;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextColourBase<T> {
    Colour(T),
}

pub type TextColour = TextColourBase<Colour>;

impl ClassName for TextColour {
    fn has_class_name(self) -> bool {
        self != TextColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            TextColour::Colour(Colour::Default) => "".to_string(),
            TextColour::Colour(color) => format!("text-{}", color),
        })
    }
}

impl Display for TextColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextContentColourBase<T> {
    Colour(T),
}
