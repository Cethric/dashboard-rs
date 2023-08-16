use std::fmt::{Display, Formatter};

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::responsive::{Responsive, ResponsiveVec};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DividerOrientation {
    Default,
    Horizontal,
    Vertical,
}

impl ClassName for DividerOrientation {
    fn has_class_name(self) -> bool {
        self != DividerOrientation::Default
    }

    fn class_name(self) -> String {
        match self {
            DividerOrientation::Default => "".to_string(),
            DividerOrientation::Horizontal => "divider-horizontal".to_string(),
            DividerOrientation::Vertical => "divider-vertical".to_string(),
        }
    }
}

impl Display for DividerOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Divider(
    cx: Scope,
    #[prop(default = ResponsiveVec(vec![]))] orientation: ResponsiveVec<
        Responsive<DividerOrientation>,
    >,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {cx,
        <div class=format!("divider{}", orientation)>{if let Some(children) = children {children(cx).into_view(cx)} else {().into_view(cx)}}</div>
    }
}
