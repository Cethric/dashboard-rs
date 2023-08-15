use leptos::*;

#[component]
pub fn Countdown(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    assert!(value.get() >= 0 && value.get() < 100);
    view! {cx,
        <div class="countdown">
            <span style=format!("--value:{}", value.get())/>
        </div>
    }
}
