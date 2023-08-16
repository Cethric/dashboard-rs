use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DropdownPosition {
    Default,
    Top,
    Bottom,
    Left,
    Right,
}

impl ClassName for DropdownPosition {
    fn has_class_name(self) -> bool {
        self != DropdownPosition::Default
    }

    fn class_name(self) -> String {
        match self {
            DropdownPosition::Default => "".to_string(),
            DropdownPosition::Top => "dropdown-top".to_string(),
            DropdownPosition::Bottom => "dropdown-bottom".to_string(),
            DropdownPosition::Left => "dropdown-left".to_string(),
            DropdownPosition::Right => "dropdown-right".to_string(),
        }
    }
}

impl Display for DropdownPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DropdownAlign {
    Default,
    Start,
    End,
}

impl ClassName for DropdownAlign {
    fn has_class_name(self) -> bool {
        self != DropdownAlign::Default
    }

    fn class_name(self) -> String {
        String::from(match self {
            DropdownAlign::Default => "",
            DropdownAlign::Start => "dropdown-start",
            DropdownAlign::End => "dropdown-end",
        })
    }
}

impl Display for DropdownAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
fn DropdownContent(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="dropdown-content shadow z-[1] bg-base-300 rounded-box w-52">
            {children(cx)}
        </div>
    }
}

#[component]
pub fn Dropdown(
    cx: Scope,
    label: &'static str,
    #[prop(default = DropdownPosition::Default)] position: DropdownPosition,
    #[prop(default = DropdownAlign::Default)] align: DropdownAlign,
    #[prop(default = false)] hover: bool,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <details class=format!("dropdown{}{}{}", if hover {" dropdown-hover"} else {""}, position, align)>
            <summary class="btn">{label}</summary>
            <DropdownContent>{children(cx)}</DropdownContent>
        </details>
    }
}
