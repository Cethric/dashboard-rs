use leptos::*;

use crate::colour::Colour;
use crate::common::TextBackgroundColour;

#[slot]
pub struct FooterColumnTitle {
    children: ChildrenFn,
}

#[component]
pub fn FooterColumn(
    cx: Scope,
    #[prop(optional)] title: Option<FooterColumnTitle>,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div>
            {if let Some(title) = title {(view! {cx, <span class="footer-title">{(title.children)(cx)}</span>}).into_view(cx)} else {().into_view(cx)}}
            {children(cx)}
        </div>
    }
}

#[component]
pub fn Footer(
    cx: Scope,
    #[prop(default=TextBackgroundColour::Colour(Colour::Default))] colour: TextBackgroundColour,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <footer class=format!("footer p-10{}", colour)>{children(cx)}</footer>
    }
}
