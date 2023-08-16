use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProgressColourBase<T> {
    Colour(T),
}

pub type ProgressColour = ProgressColourBase<Colour>;

impl ClassName for ProgressColour {
    fn has_class_name(self) -> bool {
        self != ProgressColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        match self {
            ProgressColour::Colour(Colour::Default) => "".to_string(),
            ProgressColour::Colour(color) => format!("progress-{}", color),
        }
    }
}

impl Display for ProgressColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Progress(
    cx: Scope,
    #[prop(default=ProgressColour::Colour(Colour::Default))] colour: ProgressColour,
    #[prop(default = 0)] min: i32,
    #[prop(default = 100)] max: i32,
    #[prop(optional)] progress: Option<ReadSignal<i32>>,
) -> impl IntoView {
    if let Some(progress) = progress {
        assert!(progress.get() >= min && progress.get() <= max);
        view! {cx, <progress class=format!("progress{}", colour) min=min max=max progress=progress.get()/>}
    } else {
        view! {cx, <progress class=format!("progress{}", colour) min=min max=max />}
    }
}
