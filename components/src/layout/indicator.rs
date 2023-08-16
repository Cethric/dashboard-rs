use std::fmt::Display;

use leptos::*;

use crate::align::{HorizontalAlign, VerticalAlign};
use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;
use crate::data_display::badge::BadgeColour;
use crate::responsive::{Responsive, ResponsiveVec};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IndicatorAlign {
    Align(HorizontalAlign, VerticalAlign),
    Horizontal(HorizontalAlign),
    Vertical(VerticalAlign),
}

impl ClassName for IndicatorAlign {
    fn has_class_name(self) -> bool {
        true
    }

    fn class_name(self) -> String {
        match self {
            IndicatorAlign::Align(horizontal, vertical) => format!(
                "{}{}",
                IndicatorAlign::Horizontal(horizontal),
                IndicatorAlign::Vertical(vertical)
            ),
            IndicatorAlign::Horizontal(horizontal) => format!("indicator-{}", horizontal),
            IndicatorAlign::Vertical(vertical) => format!("indicator-{}", vertical),
        }
    }
}

impl Display for IndicatorAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub type ResponsiveIndicatorAlign = ResponsiveVec<Responsive<IndicatorAlign>>;

#[slot]
pub struct IndicatorItem {
    children: ChildrenFn,
}

#[component]
pub fn Indicator(
    cx: Scope,
    #[prop(default=BadgeColour::Colour(Colour::Default))] colour: BadgeColour,
    #[prop(default=ResponsiveVec(vec![]))] align: ResponsiveIndicatorAlign,
    indicator_item: IndicatorItem,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class="indicator">
            <span class=format!("indicator-item{} badge{}", align, colour)>{(indicator_item.children)(cx)}</span>
            {children(cx)}
        </div>
    }
}
