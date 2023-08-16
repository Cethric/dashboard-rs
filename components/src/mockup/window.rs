use leptos::*;

#[component]
pub fn MockupWindow(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="mockup-window">
            <div class="flex justify-center px-4 py-16">
                {children(cx)}
            </div>
        </div>
    }
}
