use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;
use crate::orientation::Orientation;
use crate::responsive::{Responsive, ResponsiveVec};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StepOrientation {
    Orientation(Orientation),
}

impl ClassName for StepOrientation {
    fn has_class_name(self) -> bool {
        self != StepOrientation::Orientation(Orientation::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            StepOrientation::Orientation(Orientation::Default) => "".to_string(),
            StepOrientation::Orientation(orientation) => format!("step-{}", orientation),
        })
    }
}

impl Display for StepOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub type ResponsiveStepOrientation = ResponsiveVec<Responsive<StepOrientation>>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StepColour {
    Colour(Colour),
}

impl ClassName for StepColour {
    fn has_class_name(self) -> bool {
        self != StepColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            StepColour::Colour(Colour::Default) => "".to_string(),
            StepColour::Colour(color) => format!("step-{}", color),
        })
    }
}

impl Display for StepColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[slot]
pub struct Step {
    children: ChildrenFn,
    colour: StepColour,
    content: Option<&'static str>,
}

#[component]
pub fn Steps(
    cx: Scope,
    #[prop(default = ResponsiveVec(vec![]))] orientation: ResponsiveStepOrientation,
    step: Vec<Step>,
) -> impl IntoView {
    view! {cx,
        <ul class=format!("steps{}", orientation)>
            {
                step
                    .iter()
                    .map(|item|view! {cx, <li class=format!("step{}", item.colour) data-content=item.content>{(item.children)(cx)}</li>})
                    .collect_view(cx)
            }
        </ul>
    }
}
