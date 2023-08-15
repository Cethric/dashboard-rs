use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;
use crate::responsive::{Responsive, ResponsiveVec};
use crate::size::Size;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ButtonColourBase<T> {
    Colour(T),
    Ghost(),
    Link(),
}

pub type ButtonColour = ButtonColourBase<Colour>;

impl ClassName for ButtonColour {
    fn has_class_name(self) -> bool {
        self != ButtonColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            ButtonColour::Colour(Colour::Default) => "".to_string(),
            ButtonColour::Colour(color) => format!("btn-{}", color),
            ButtonColour::Ghost() => "btn-ghost".to_string(),
            ButtonColour::Link() => "btn-link".to_string(),
        })
    }
}

impl Display for ButtonColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ButtonSizeBase<T> {
    Size(T),
}

pub type ButtonSize = ButtonSizeBase<Size>;

impl ClassName for ButtonSize {
    fn has_class_name(self) -> bool {
        self != ButtonSize::Size(Size::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            ButtonSize::Size(Size::Default) => "".to_string(),
            ButtonSize::Size(size) => format!("btn-{}", size),
        })
    }
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ButtonShape {
    Square,
    Circle,
}

impl ClassName for ButtonShape {
    fn has_class_name(self) -> bool {
        true
    }

    fn class_name(self) -> String {
        String::from(match self {
            ButtonShape::Square => "btn-square",
            ButtonShape::Circle => "btn-circle",
        })
    }
}

impl Display for ButtonShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ButtonType {
    Default,
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    pub fn button_type(self) -> &'static str {
        match self {
            ButtonType::Default => "",
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        }
    }
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.button_type())
    }
}

#[component]
pub fn Button(
    cx: Scope,
    #[prop(default = ButtonColour::Colour(Colour::Default))] colour: ButtonColour,
    #[prop(default = ButtonType::Default)] button_type: ButtonType,
    #[prop(default=ResponsiveVec(vec![]))] size: ResponsiveVec<Responsive<ButtonSize>>,
    #[prop(default=ResponsiveVec(vec![]))] shape: ResponsiveVec<Responsive<ButtonShape>>,
    #[prop(default = false)] active: bool,
    #[prop(default = false)] outline: bool,
    #[prop(default = false)] wide: bool,
    #[prop(default = false)] block: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] no_animation: bool,
    #[prop(default = false)] glass: bool,
    children: Children,
) -> impl IntoView {
    view! {cx,
    <button
        disabled=disabled
        class={
            format!(
                "btn{}{}{}{}{}{}{}{}{}{}",
                if active {" btn-active"} else {""},
                if outline {" btn-outline"} else {""},
                if wide {" btn-wide"} else {""},
                if block {" btn-block"} else {""},
                if disabled {" btn-disabled"} else {""},
                if no_animation {" no-animation"} else {""},
                if glass {" glass"} else {""},
                colour,
                size,
                shape,
            )
        }
        type=format!("{}", button_type)
    >
        {children(cx)}
    </button>}
}
