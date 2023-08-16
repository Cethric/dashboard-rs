use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::orientation::Orientation;
use crate::responsive::{Responsive, ResponsiveVec};
use crate::size::Size;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MenuOrientation {
    Orientation(Orientation),
}

impl ClassName for MenuOrientation {
    fn has_class_name(self) -> bool {
        self != MenuOrientation::Orientation(Orientation::Default)
    }

    fn class_name(self) -> String {
        match self {
            MenuOrientation::Orientation(Orientation::Default) => "".to_string(),
            MenuOrientation::Orientation(orientation) => format!("menu-{}", orientation),
        }
    }
}

impl Display for MenuOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub type ResponsiveMenuOrientation = ResponsiveVec<Responsive<MenuOrientation>>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MenuSize {
    Size(Size),
}

impl ClassName for MenuSize {
    fn has_class_name(self) -> bool {
        self != MenuSize::Size(Size::Default)
    }

    fn class_name(self) -> String {
        match self {
            MenuSize::Size(Size::Default) => "".to_string(),
            MenuSize::Size(orientation) => format!("menu-{}", orientation),
        }
    }
}

impl Display for MenuSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub type ResponsiveMenuSize = ResponsiveVec<Responsive<MenuSize>>;

#[derive(Debug, Clone)]
pub enum MenuItem {
    Title(View),
    Item(View),
    TitleWithChildren(View, MaybeSignal<bool>, Vec<MenuItem>),
}
#[component]
fn RenderMenuItems(cx: Scope, items: Vec<MenuItem>) -> impl IntoView {
    view! {cx,
        {
            items.iter().map(|item| {
                match item {
                    MenuItem::Title(title) => view!(cx, <li class="menu-title">{title.into_view(cx)}</li>),
                    MenuItem::Item(children) => view!(cx, <li>{children.into_view(cx)}</li>),
                    MenuItem::TitleWithChildren(title, open, children) => view!(cx,
                        <li>
                            <details open=open.get()>
                                <summary>{title.into_view(cx)}</summary>
                                <ul>
                                    <RenderMenuItems items=children.to_vec()/>
                                </ul>
                            </details>
                        </li>
                    )
                }
            }).collect_view(cx)
        }
    }
}

#[component]
pub fn Menu(
    cx: Scope,
    #[prop(default = ResponsiveVec(vec![]))] size: ResponsiveMenuSize,
    #[prop(default = ResponsiveVec(vec![]))] orientation: ResponsiveMenuOrientation,
    items: Vec<MenuItem>,
) -> impl IntoView {
    view! {cx,
        <ul class=format!("menu{}{}",size, orientation)>
            <RenderMenuItems items=items/>
        </ul>
    }
}
