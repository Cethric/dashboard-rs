use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::orientation::Orientation;
use crate::responsive::{Responsive, ResponsiveVec};

#[slot]
pub struct StatTitle {
    children: ChildrenFn,
}
#[slot]
pub struct StatValue {
    children: ChildrenFn,
}
#[slot]
pub struct StatDesc {
    children: ChildrenFn,
}
#[slot]
pub struct StatFigure {
    children: ChildrenFn,
}
#[slot]
pub struct StatActions {
    children: ChildrenFn,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StatOrientation {
    Orientation(Orientation),
}

impl ClassName for StatOrientation {
    fn has_class_name(self) -> bool {
        self != StatOrientation::Orientation(Orientation::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            StatOrientation::Orientation(Orientation::Default) => "".to_string(),
            StatOrientation::Orientation(orientation) => format!("stat-{}", orientation),
        })
    }
}

impl Display for StatOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Stat(
    cx: Scope,
    #[prop(optional)] stat_title: Option<StatTitle>,
    #[prop(optional)] stat_value: Option<StatValue>,
    #[prop(optional)] stat_desc: Option<StatDesc>,
    #[prop(optional)] stat_figure: Option<StatFigure>,
    #[prop(optional)] stat_actions: Option<StatActions>,
) -> impl IntoView {
    view! {cx,
        <div class="stat">
            {if let Some(stat_figure) = stat_figure {(view! {cx, <div class="stat-figure">{(stat_figure.children)(cx)}</div>}).into_view(cx)} else {().into_view(cx)}}
            {if let Some(stat_title) = stat_title {(view! {cx, <div class="stat-title">{(stat_title.children)(cx)}</div>}).into_view(cx)} else {().into_view(cx)}}
            {if let Some(stat_value) = stat_value {(view! {cx, <div class="stat-value">{(stat_value.children)(cx)}</div>}).into_view(cx)} else {().into_view(cx)}}
            {if let Some(stat_desc) = stat_desc {(view! {cx, <div class="stat-desc">{(stat_desc.children)(cx)}</div>}).into_view(cx)} else {().into_view(cx)}}
            {if let Some(stat_actions) = stat_actions {(view! {cx, <div class="stat-actions">{(stat_actions.children)(cx)}</div>}).into_view(cx)} else {().into_view(cx)}}
        </div>
    }
}

#[component]
pub fn Stats(
    cx: Scope,
    #[prop(default = ResponsiveVec(vec![]))] orientation: ResponsiveVec<
        Responsive<StatOrientation>,
    >,
    children: Children,
) -> impl IntoView {
    view! {cx, <div class=format!("stats{}", orientation)>{children(cx)}</div>}
}
