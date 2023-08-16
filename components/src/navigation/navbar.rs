use leptos::*;

use crate::colour::Colour;
use crate::common::TextBackgroundColour;

#[slot]
pub struct NavbarStart {
    children: ChildrenFn,
}
#[slot]
pub struct NavbarCenter {
    children: ChildrenFn,
}
#[slot]
pub struct NavbarEnd {
    children: ChildrenFn,
}

#[component]
pub fn Navbar(
    cx: Scope,
    #[prop(default=TextBackgroundColour::Colour(Colour::Default))] colour: TextBackgroundColour,
    #[prop(optional)] navbar_start: Option<NavbarStart>,
    #[prop(optional)] navbar_center: Option<NavbarCenter>,
    #[prop(optional)] navbar_end: Option<NavbarEnd>,
) -> impl IntoView {
    view! {cx,
        <nav class=format!("navbar{}", colour)>
            {if let Some(navbar_start) = navbar_start {(view! {cx, <div class="navbar-start">{(navbar_start.children)(cx)}</div>}).into_view(cx)} else {().into_view(cx)}}
            {if let Some(navbar_center) = navbar_center {(view! {cx, <div class="navbar-center">{(navbar_center.children)(cx)}</div>}).into_view(cx)} else {().into_view(cx)}}
            {if let Some(navbar_end) = navbar_end {(view! {cx, <div class="navbar-end">{(navbar_end.children)(cx)}</div>}).into_view(cx)} else {().into_view(cx)}}
        </nav>
    }
}
