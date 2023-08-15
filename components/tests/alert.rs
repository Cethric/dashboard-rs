#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    #[allow(unused_imports)]
    use components::colour::Colour;
    use components::data_display::alert::*;

    #[test_case(AlertColour::Colour(Colour::Default); "Default")]
    #[test_case(AlertColour::Colour(Colour::Neutral); "Neutral")]
    #[test_case(AlertColour::Colour(Colour::Primary); "Primary")]
    #[test_case(AlertColour::Colour(Colour::Secondary); "Secondary")]
    #[test_case(AlertColour::Colour(Colour::Accent); "Accent")]
    #[test_case(AlertColour::Colour(Colour::Info); "Info")]
    #[test_case(AlertColour::Colour(Colour::Success); "Success")]
    #[test_case(AlertColour::Colour(Colour::Warning); "Warning")]
    #[test_case(AlertColour::Colour(Colour::Error); "Error")]
    pub fn test_alert(colour: AlertColour) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Alert colour=colour>button</Alert>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-alert-start--><!--leptos-view|components-src-data_display-alert.rs-40|open--><div class=\"alert{}\" id=\"_0-2\"><!--leptos-view|<Alert/>-children|open--><!--hk=_0-3o|leptos--start-->button<!--hk=_0-3c|leptos--end--><!--leptos-view|<Alert/>-children|close--></div><!--leptos-view|components-src-data_display-alert.rs-40|close--><!--hk=_0-1c|leptos-alert-end-->",
                    colour
                )
            );
        }).dispose();
    }
}
