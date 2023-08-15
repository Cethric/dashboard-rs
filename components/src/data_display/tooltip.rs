use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;
use crate::responsive::{Responsive, ResponsiveVec};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TooltipColourBase<T> {
    Colour(T),
}

pub type TooltipColour = TooltipColourBase<Colour>;

impl ClassName for TooltipColour {
    fn has_class_name(self) -> bool {
        self != TooltipColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            TooltipColour::Colour(Colour::Default) => "".to_string(),
            TooltipColour::Colour(color) => format!("tooltip-{}", color),
        })
    }
}

impl Display for TooltipColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TooltipSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl ClassName for TooltipSide {
    fn has_class_name(self) -> bool {
        true
    }

    fn class_name(self) -> String {
        String::from(match self {
            TooltipSide::Top => "tooltip-top".to_string(),
            TooltipSide::Bottom => "tooltip-bottom".to_string(),
            TooltipSide::Left => "tooltip-left".to_string(),
            TooltipSide::Right => "tooltip-right".to_string(),
        })
    }
}

impl Display for TooltipSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Tooltip(
    cx: Scope,
    colour: TooltipColour,
    #[prop(default = ResponsiveVec(vec![]))] side: ResponsiveVec<Responsive<TooltipSide>>,
    tooltip: &'static str,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class=format!("tooltip{}{}", colour, side) data-tip=tooltip>
            {children(cx)}
        </div>
    }
}
