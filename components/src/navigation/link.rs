use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LinkColour {
    Colour(Colour),
}

impl ClassName for LinkColour {
    fn has_class_name(self) -> bool {
        self != LinkColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        match self {
            LinkColour::Colour(Colour::Default) => "".to_string(),
            LinkColour::Colour(color) => format!("link-{}", color),
        }
    }
}

impl Display for LinkColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Link(
    cx: Scope,
    #[prop(default = LinkColour::Colour(Colour::Default))] colour: LinkColour,
    #[prop(default = false)] hover: bool,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <a class=format!("link{}{}", colour, if hover {"link-hover"} else {""})>
            {children(cx)}
        </a>
    }
}
