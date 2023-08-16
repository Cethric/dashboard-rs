use leptos::*;

#[component]
pub fn Countdown(cx: Scope, value: MaybeSignal<i32>) -> impl IntoView {
    assert!(value.get() >= 0 && value.get() < 100);
    view! {cx,
        <div class="countdown">
            <span style=move || format!("--value:{}", value.derive_signal(cx).get())/>
        </div>
    }
}
