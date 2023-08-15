use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CollapseIcon {
    Default,
    Arrow,
    Plus,
}

impl ClassName for CollapseIcon {
    fn has_class_name(self) -> bool {
        self != CollapseIcon::Default
    }

    fn class_name(self) -> String {
        String::from(match self {
            CollapseIcon::Default => "",
            CollapseIcon::Arrow => "collapse-arrow",
            CollapseIcon::Plus => "collapse-plus",
        })
    }
}

impl Display for CollapseIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[slot]
pub struct CollapseControl {
    children: ChildrenFn,
}
#[slot]
pub struct CollapseTitle {
    children: ChildrenFn,
}
#[slot]
pub struct CollapseContent {
    children: ChildrenFn,
}

#[component]
pub fn Collapse(
    cx: Scope,
    collapse_title: CollapseTitle,
    collapse_content: CollapseContent,
    #[prop(optional)] collapse_control: Option<CollapseControl>,
    #[prop(default = CollapseIcon::Default)] icon: CollapseIcon,
) -> impl IntoView {
    view! { cx,
        <div tabindex={if collapse_control.is_none() {0} else {-1}} class=format!("collapse{}", icon)>
            {if let Some(collapse_control) = &collapse_control { (collapse_control.children)(cx).into_view(cx) } else {().into_view(cx)}}
            <div class="collapse-title">{(collapse_title.children)(cx)}</div>
            <div class="collapse-content">{(collapse_content.children)(cx)}</div>
        </div>
    }
}

#[component]
pub fn CollapseDetails(
    cx: Scope,
    collapse_title: CollapseTitle,
    collapse_content: CollapseContent,
    #[prop(default = CollapseIcon::Default)] icon: CollapseIcon,
) -> impl IntoView {
    view! { cx,
        <details class=format!("collapse{}", icon)>
            <summary class="collapse-title">{(collapse_title.children)(cx)}</summary>
            <div class="collapse-content">{(collapse_content.children)(cx)}</div>
        </details>
    }
}
