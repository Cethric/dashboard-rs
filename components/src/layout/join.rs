use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::orientation::Orientation;
use crate::responsive::{Responsive, ResponsiveVec};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JoinOrientation {
    Orientation(Orientation),
}

impl ClassName for JoinOrientation {
    fn has_class_name(self) -> bool {
        self != JoinOrientation::Orientation(Orientation::Default)
    }

    fn class_name(self) -> String {
        match self {
            JoinOrientation::Orientation(Orientation::Default) => "".to_string(),
            JoinOrientation::Orientation(orientation) => format!("join-{}", orientation),
        }
    }
}

impl Display for JoinOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub type ResponsiveJoinOrientation = ResponsiveVec<Responsive<JoinOrientation>>;

#[slot]
pub struct JoinItem {
    children: ChildrenFn,
}

#[component]
pub fn Join(
    cx: Scope,
    #[prop(default = ResponsiveVec(vec![]))] orientation: ResponsiveJoinOrientation,
    join_item: Vec<JoinItem>,
) -> impl IntoView {
    view! {cx,
        <div class=format!("join{}", orientation)>
            {join_item.iter().map(|item| view! {cx, class="join-item", <>{(item.children)(cx)}</>}).collect_view(cx)}
        </div>
    }
}
