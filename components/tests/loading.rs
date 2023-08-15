#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    use components::class_name::ClassName;
    #[allow(unused_imports)]
    use components::colour::Colour;
    use components::data_display::loading::*;
    use components::responsive::{Responsive, ResponsiveVec};
    use components::size::Size;
    use components::typography::TextColour;

    #[test_case(LoadingSize::Size(Size::Default); "Default")]
    #[test_case(LoadingSize::Size(Size::Large); "Large")]
    #[test_case(LoadingSize::Size(Size::Medium); "Medium")]
    #[test_case(LoadingSize::Size(Size::Small); "Small")]
    #[test_case(LoadingSize::Size(Size::ExtraSmall); "ExtraSmall")]
    pub fn test_loading_size(size: LoadingSize) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Loading size=ResponsiveVec(vec![Responsive::Default(size), Responsive::Small(size), Responsive::Medium(size), Responsive::Large(size), Responsive::ExtraLarge(size), Responsive::ExtraExtraLarge(size)])/>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-loading-start--><!--leptos-view|components-src-data_display-loading.rs-79|open--><span class=\"loading{}{}{}{}{}{}\" id=\"_0-2\"></span><!--leptos-view|components-src-data_display-loading.rs-79|close--><!--hk=_0-1c|leptos-loading-end-->",
                    if size == LoadingSize::Size(Size::Default) {"".to_string()} else {format!("{}", size)},
                    if size == LoadingSize::Size(Size::Default) {"".to_string()} else {format!(" sm:{}", size.class_name())},
                    if size == LoadingSize::Size(Size::Default) {"".to_string()} else {format!(" md:{}", size.class_name())},
                    if size == LoadingSize::Size(Size::Default) {"".to_string()} else {format!(" lg:{}", size.class_name())},
                    if size == LoadingSize::Size(Size::Default) {"".to_string()} else {format!(" xl:{}", size.class_name())},
                    if size == LoadingSize::Size(Size::Default) {"".to_string()} else {format!(" 2xl:{}", size.class_name())}
                )
            );
        }).dispose();
    }

    #[test_case(LoadingVariant::Default; "Default")]
    #[test_case(LoadingVariant::Spinner; "Spinner")]
    #[test_case(LoadingVariant::Dots; "Dots")]
    #[test_case(LoadingVariant::Ring; "Ring")]
    #[test_case(LoadingVariant::Ball; "Ball")]
    #[test_case(LoadingVariant::Bars; "Bars")]
    #[test_case(LoadingVariant::Infinity; "Infinity")]
    pub fn test_loading_variant(variant: LoadingVariant) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Loading variant=variant/>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-loading-start--><!--leptos-view|components-src-data_display-loading.rs-79|open--><span class=\"loading{}\" id=\"_0-2\"></span><!--leptos-view|components-src-data_display-loading.rs-79|close--><!--hk=_0-1c|leptos-loading-end-->",
                    if variant == LoadingVariant::Default {"".to_string()} else {format!("{}", variant)}
                )
            );
        }).dispose();
    }

    #[test_case(TextColour::Colour(Colour::Default); "Default")]
    #[test_case(TextColour::Colour(Colour::Primary); "Primary")]
    #[test_case(TextColour::Colour(Colour::Secondary); "Secondary")]
    #[test_case(TextColour::Colour(Colour::Accent); "Accent")]
    #[test_case(TextColour::Colour(Colour::Neutral); "Neutral")]
    #[test_case(TextColour::Colour(Colour::Info); "Info")]
    #[test_case(TextColour::Colour(Colour::Success); "Success")]
    #[test_case(TextColour::Colour(Colour::Warning); "Warning")]
    #[test_case(TextColour::Colour(Colour::Error); "Error")]
    pub fn test_loading_colour(colour: TextColour) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Loading colour=colour/>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-loading-start--><!--leptos-view|components-src-data_display-loading.rs-79|open--><span class=\"loading{}\" id=\"_0-2\"></span><!--leptos-view|components-src-data_display-loading.rs-79|close--><!--hk=_0-1c|leptos-loading-end-->",
                    colour
                )
            );
        }).dispose();
    }
}
