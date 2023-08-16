use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::responsive::{Responsive, ResponsiveVec};
use crate::size::Size;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TabSize {
    Size(Size),
}

impl ClassName for TabSize {
    fn has_class_name(self) -> bool {
        self != TabSize::Size(Size::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            TabSize::Size(Size::Default) => "".to_string(),
            TabSize::Size(orientation) => format!("menu-{}", orientation),
        })
    }
}

impl Display for TabSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub type ResponsiveTabSize = ResponsiveVec<Responsive<TabSize>>;

#[slot]
pub struct TabItem {
    children: ChildrenFn,
    #[prop(default = false)]
    active: bool,
    #[prop(default = false)]
    disabled: bool,
    #[prop(default = false)]
    bordered: bool,
    #[prop(default = false)]
    lifted: bool,
    #[prop(default = ResponsiveVec(vec![]))]
    size: ResponsiveTabSize,
}

#[component]
pub fn Tabs(
    cx: Scope,
    #[prop(default = false)] boxed: bool,
    tab_item: Vec<TabItem>,
) -> impl IntoView {
    view! {cx,
        <div class=format!("tabs{}", if boxed {" tabs-boxed"} else {""})>
            {
                tab_item
                    .iter()
                    .map(|item|view! {cx, <a disabled=item.disabled class=format!("tab{}{}{}{}", if item.active {" tab-active"} else {""}, if item.bordered {" tab-bordered"} else {""}, if item.lifted {" tab-lifted"} else {""}, item.size)>{(item.children)(cx)}</a>})
                    .collect_view(cx)
            }
        </div>
    }
}
