use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::{BackgroundColour, BorderColour, Colour};
use crate::style::SizeUnit;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RadialProgressColourBase<T> {
    Colour(T),
}

pub type RadialProgressColour = RadialProgressColourBase<Colour>;

impl ClassName for RadialProgressColour {
    fn has_class_name(self) -> bool {
        self != RadialProgressColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            RadialProgressColour::Colour(Colour::Default) => "".to_string(),
            RadialProgressColour::Colour(colour) => format!(
                "{}{}border-4{}",
                BackgroundColour::Colour(colour),
                colour.to_content_colour(),
                BorderColour::Colour(colour)
            ),
        })
    }
}

impl Display for RadialProgressColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn RadialProgress(
    cx: Scope,
    #[prop(default=RadialProgressColour::Colour(Colour::Default))] colour: RadialProgressColour,
    #[prop(default = SizeUnit::Em(4.0))] size: SizeUnit,
    #[prop(default = SizeUnit::Em(0.4))] thickness: SizeUnit,
    progress: ReadSignal<i32>,
) -> impl IntoView {
    assert!(progress.get() >= 0 && progress.get() <= 100);
    view! {cx,
        <div class=format!("progress{}", colour) style=format!("--value:{}; --size:{}; --thickness:{};", progress.get(), size, thickness)>
            {format!("{}%", progress.get())}
        </div>
    }
}
