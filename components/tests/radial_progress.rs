#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    #[allow(unused_imports)]
    use components::colour::Colour;
    use components::data_display::radial_progress::*;

    #[test_case(RadialProgressColour::Colour(Colour::Default); "Default")]
    #[test_case(RadialProgressColour::Colour(Colour::Neutral); "Neutral")]
    #[test_case(RadialProgressColour::Colour(Colour::Primary); "Primary")]
    #[test_case(RadialProgressColour::Colour(Colour::Secondary); "Secondary")]
    #[test_case(RadialProgressColour::Colour(Colour::Accent); "Accent")]
    #[test_case(RadialProgressColour::Colour(Colour::Info); "Info")]
    #[test_case(RadialProgressColour::Colour(Colour::Success); "Success")]
    #[test_case(RadialProgressColour::Colour(Colour::Warning); "Warning")]
    #[test_case(RadialProgressColour::Colour(Colour::Error); "Error")]
    pub fn test_radial_progress_colour(colour: RadialProgressColour) {
        create_scope(create_runtime(), move |cx| {
            let (value,_) = create_signal(cx, 10);
            let view = view! {cx, <RadialProgress colour=colour progress=MaybeSignal::Dynamic(value.into())/>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-radial-progress-start--><!--leptos-view|components-src-data_display-radial_progress.rs-49|open--><div class=\"progress{}\" style=\"--value:{}; --size:4em; --thickness:0.4em;\" id=\"_0-2\">{}%</div><!--leptos-view|components-src-data_display-radial_progress.rs-49|close--><!--hk=_0-1c|leptos-radial-progress-end-->",
                    colour,
                    value.get(),
                    value.get()
                )
            );
        }).dispose();
    }
}
