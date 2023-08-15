use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;
use crate::responsive::{Responsive, ResponsiveVec};
use crate::size::Size;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BadgeColourBase<T> {
    Colour(T),
    Ghost(),
}

pub type BadgeColour = BadgeColourBase<Colour>;

impl ClassName for BadgeColour {
    fn has_class_name(self) -> bool {
        self != BadgeColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            BadgeColour::Colour(Colour::Default) => "".to_string(),
            BadgeColour::Colour(color) => format!("badge-{}", color),
            BadgeColour::Ghost() => "badge-ghost".to_string(),
        })
    }
}

impl Display for BadgeColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BadgeSizeBase<T> {
    Size(T),
}

pub type BadgeSize = BadgeSizeBase<Size>;

impl ClassName for BadgeSize {
    fn has_class_name(self) -> bool {
        self != BadgeSize::Size(Size::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            BadgeSize::Size(Size::Default) => "".to_string(),
            BadgeSize::Size(size) => format!("badge-{}", size),
        })
    }
}

impl Display for BadgeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Badge(
    cx: Scope,
    #[prop(default = false)] outline: bool,
    #[prop(default = BadgeColour::Colour(Colour::Default))] colour: BadgeColour,
    #[prop(default = ResponsiveVec(vec![]))] size: ResponsiveVec<Responsive<BadgeSize>>,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <span
            class={
                format!(
                    "badge{}{}{}",
                    if outline {" badge-outline"} else {""},
                    colour,
                    size
                )
            }
        >
            {children(cx)}
        </span>
    }
}
