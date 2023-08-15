#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    #[allow(unused_imports)]
    use components::colour::Colour;
    use components::data_display::progress::*;

    #[test_case(ProgressColour::Colour(Colour::Default); "Default")]
    #[test_case(ProgressColour::Colour(Colour::Neutral); "Neutral")]
    #[test_case(ProgressColour::Colour(Colour::Primary); "Primary")]
    #[test_case(ProgressColour::Colour(Colour::Secondary); "Secondary")]
    #[test_case(ProgressColour::Colour(Colour::Accent); "Accent")]
    #[test_case(ProgressColour::Colour(Colour::Info); "Info")]
    #[test_case(ProgressColour::Colour(Colour::Success); "Success")]
    #[test_case(ProgressColour::Colour(Colour::Warning); "Warning")]
    #[test_case(ProgressColour::Colour(Colour::Error); "Error")]
    pub fn test_progress_colour(colour: ProgressColour) {
        create_scope(create_runtime(), move |cx| {
            let (value,_) = create_signal(cx, 0);
            let view = view! {cx, <Progress colour=colour progress=value/>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-progress-start--><!--leptos-view|components-src-data_display-progress.rs-44|open--><progress class=\"progress{}\" min=\"0\" max=\"100\" progress=\"0\" id=\"_0-2\"></progress><!--leptos-view|components-src-data_display-progress.rs-44|close--><!--hk=_0-1c|leptos-progress-end-->",
                    colour
                )
            );
        }).dispose();
    }

    #[test]
    pub fn test_progress_indeterminate() {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Progress />};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-progress-start--><!--leptos-view|components-src-data_display-progress.rs-46|open--><progress class=\"progress\" min=\"0\" max=\"100\" id=\"_0-2\"></progress><!--leptos-view|components-src-data_display-progress.rs-46|close--><!--hk=_0-1c|leptos-progress-end-->",
                )
            );
        }).dispose();
    }
}
