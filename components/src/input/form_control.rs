use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputColourBase<T> {
    Colour(T),
}

pub type InputColour = InputColourBase<Colour>;

impl ClassName for InputColour {
    fn has_class_name(self) -> bool {
        self != InputColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        match self {
            InputColour::Colour(Colour::Default) => "".to_string(),
            InputColour::Colour(color) => format!("input-{}", color),
        }
    }
}

impl Display for InputColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn FormControl(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="form-control">
            <label class="label">
                <span class="label-text">Pick a file</span>
                <span class="label-text-alt">Alt label</span>
            </label>
            <input type="file" class="file-input file-input-bordered w-full max-w-xs" />
            <label class="label">
                <span class="label-text-alt">Alt label</span>
                <span class="label-text-alt">Alt label</span>
            </label>
        </div>
    }
}
