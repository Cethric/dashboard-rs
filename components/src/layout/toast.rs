use std::fmt::Display;

use leptos::*;

use crate::align::{HorizontalAlign, VerticalAlign};
use crate::class_name::{fmt_class_name, ClassName};
use crate::responsive::{Responsive, ResponsiveVec};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ToastAlign {
    Align(HorizontalAlign, VerticalAlign),
    Horizontal(HorizontalAlign),
    Vertical(VerticalAlign),
}

impl ClassName for ToastAlign {
    fn has_class_name(self) -> bool {
        true
    }

    fn class_name(self) -> String {
        match self {
            ToastAlign::Align(horizontal, vertical) => format!(
                "{}{}",
                ToastAlign::Horizontal(horizontal),
                ToastAlign::Vertical(vertical)
            ),
            ToastAlign::Horizontal(horizontal) => format!("toast-{}", horizontal),
            ToastAlign::Vertical(vertical) => format!("toast-{}", vertical),
        }
    }
}

impl Display for ToastAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub type ResponsiveToastAlign = ResponsiveVec<Responsive<ToastAlign>>;

#[slot]
pub struct IndicatorItem {
    children: ChildrenFn,
}

#[component]
pub fn Toast(
    cx: Scope,
    #[prop(default=ResponsiveVec(vec![]))] align: ResponsiveToastAlign,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class=format!("toast{}", align)>
            {children(cx)}
        </div>
    }
}
