use leptos::*;

#[component]
pub fn Stack(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="stack">
            {children(cx)}
        </div>
    }
}
