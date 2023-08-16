use std::fmt::Display;

use leptos::*;
use leptos_icons::Icon;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;
use crate::responsive::{Responsive, ResponsiveVec};
use crate::size::Size;
use crate::typography::TextColour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BottomNavigationSize {
    Size(Size),
}

impl ClassName for BottomNavigationSize {
    fn has_class_name(self) -> bool {
        self != BottomNavigationSize::Size(Size::Default)
    }

    fn class_name(self) -> String {
        match self {
            BottomNavigationSize::Size(Size::Default) => "".to_string(),
            BottomNavigationSize::Size(size) => format!("btm-nav-{}", size),
        }
    }
}

impl Display for BottomNavigationSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub type ResponsiveBottomNavigationSize = ResponsiveVec<Responsive<BottomNavigationSize>>;

#[slot]
pub struct BottomNavigationItem {
    #[prop(default = TextColour::Colour(Colour::Default))]
    colour: TextColour,
    icon: Icon,
    active: MaybeSignal<bool>,
    disabled: MaybeSignal<bool>,
    #[prop(default = Option::None)]
    title: Option<String>,
}

#[component]
pub fn BottomNavigation(
    cx: Scope,
    bottom_navigation_item: Vec<BottomNavigationItem>,
    #[prop(default = ResponsiveVec(vec![]))] size: ResponsiveBottomNavigationSize,
) -> impl IntoView {
    view! {cx,
        <div class=format!("btm-nav{}", size)>
            {bottom_navigation_item.iter().map(
                |item| view! {cx,
                    <button disabled=item.disabled.get() class=format!("{} {}", item.colour, if item.active.get() {"active"} else {""})>
                        <Icon icon=item.icon class=format!("h-5 w-5 {}", item.colour)/>
                        {if let Some(title) = &item.title {(view! {cx, <span class="btm-nav-label">{title}</span>}).into_view(cx)} else {().into_view(cx)}}
                    </button>
                }).collect_view(cx)}
        </div>
    }
}
