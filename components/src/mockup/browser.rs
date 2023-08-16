use leptos::*;

#[component]
pub fn MockupBrowser(cx: Scope, address: &'static str, children: Children) -> impl IntoView {
    view! {cx,
        <div class="mockup-browser">
            <div class="mockup-browser-toolbar">
                <div class="input">{address}</div>
            </div>
            <div class="flex justify-center px-4 py-16">
                {children(cx)}
            </div>
        </div>
    }
}
