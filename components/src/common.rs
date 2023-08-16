use std::fmt::Display;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::{BackgroundColour, Colour};
use crate::typography::TextContentColour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextBackgroundColour {
    Colour(Colour),
}

impl ClassName for TextBackgroundColour {
    fn has_class_name(self) -> bool {
        self != TextBackgroundColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        match self {
            TextBackgroundColour::Colour(Colour::Default) => "".to_string(),
            TextBackgroundColour::Colour(colour) => {
                format!(
                    "{}{}",
                    BackgroundColour::Colour(colour),
                    TextContentColour::Colour(colour)
                )
            }
        }
    }
}

impl Display for TextBackgroundColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}
