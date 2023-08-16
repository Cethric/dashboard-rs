use leptos::*;

use crate::size::Size;
use crate::typography::TextSize;

#[component]
pub fn Breadcrumbs(
    cx: Scope,
    #[prop(default=TextSize::Size(Size::Default))] size: TextSize,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class=format!("breadcrumbs{}", size)>
            <ul>
                {children(cx)}
            </ul>
        </div>
    }
}
