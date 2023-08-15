use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AlertColourBase<T> {
    Colour(T),
}

pub type AlertColour = AlertColourBase<Colour>;

impl ClassName for AlertColour {
    fn has_class_name(self) -> bool {
        self != AlertColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            AlertColour::Colour(Colour::Default) => "".to_string(),
            AlertColour::Colour(color) => format!("alert-{}", color),
        })
    }
}

impl Display for AlertColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Alert(
    cx: Scope,
    #[prop(default = AlertColour::Colour(Colour::Default))] colour: AlertColour,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class=format!("alert{}", colour)>{children(cx)}</div>
    }
}
