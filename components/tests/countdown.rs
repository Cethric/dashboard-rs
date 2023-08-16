#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;

    use components::data_display::countdown::Countdown;

    #[test]
    pub fn test_countdown() {
        create_scope(create_runtime(), |cx| {
            let (value, _) = create_signal::<i32>(cx, 10);
            let view = view! {cx, <Countdown value=MaybeSignal::Dynamic(value.derive_signal(cx)) />};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-countdown-start--><!--leptos-view|components-src-data_display-countdown.rs-6|open--><div class=\"countdown\" id=\"_0-2\"><span style=\"--value:{}\" id=\"_0-3\"></span></div><!--leptos-view|components-src-data_display-countdown.rs-6|close--><!--hk=_0-1c|leptos-countdown-end-->",
                    10
                )
            );
        }).dispose();
    }
}
