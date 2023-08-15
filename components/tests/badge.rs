#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    use components::class_name::ClassName;
    #[allow(unused_imports)]
    use components::colour::Colour;
    use components::data_display::badge::*;
    use components::responsive::{Responsive, ResponsiveVec};
    use components::size::Size;

    #[test_case(false; "default")]
    #[test_case(true; "outline")]
    pub fn test_badge(outline: bool) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Badge outline=outline>badge</Badge>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-badge-start--><!--leptos-view|components-src-data_display-badge.rs-70|open--><span class=\"badge{}\" id=\"_0-2\"><!--leptos-view|<Badge/>-children|open--><!--hk=_0-3o|leptos--start-->badge<!--hk=_0-3c|leptos--end--><!--leptos-view|<Badge/>-children|close--></span><!--leptos-view|components-src-data_display-badge.rs-70|close--><!--hk=_0-1c|leptos-badge-end-->",
                    if outline {" badge-outline"} else {""},
                )
            );
        }).dispose();
    }

    #[test_case(BadgeColour::Colour(Colour::Default); "Default")]
    #[test_case(BadgeColour::Colour(Colour::Neutral); "Neutral")]
    #[test_case(BadgeColour::Colour(Colour::Primary); "Primary")]
    #[test_case(BadgeColour::Colour(Colour::Secondary); "Secondary")]
    #[test_case(BadgeColour::Colour(Colour::Accent); "Accent")]
    #[test_case(BadgeColour::Colour(Colour::Info); "Info")]
    #[test_case(BadgeColour::Colour(Colour::Success); "Success")]
    #[test_case(BadgeColour::Colour(Colour::Warning); "Warning")]
    #[test_case(BadgeColour::Colour(Colour::Error); "Error")]
    pub fn test_badge_variant(colour: BadgeColour) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Badge colour=colour>badge</Badge>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-badge-start--><!--leptos-view|components-src-data_display-badge.rs-70|open--><span class=\"badge{}\" id=\"_0-2\"><!--leptos-view|<Badge/>-children|open--><!--hk=_0-3o|leptos--start-->badge<!--hk=_0-3c|leptos--end--><!--leptos-view|<Badge/>-children|close--></span><!--leptos-view|components-src-data_display-badge.rs-70|close--><!--hk=_0-1c|leptos-badge-end-->",
                    colour,
                )
            );
        }).dispose();
    }

    #[test_case(BadgeSize::Size(Size::Default); "Default")]
    #[test_case(BadgeSize::Size(Size::Large); "Large")]
    #[test_case(BadgeSize::Size(Size::Medium); "Medium")]
    #[test_case(BadgeSize::Size(Size::Small); "Small")]
    #[test_case(BadgeSize::Size(Size::ExtraSmall); "ExtraSmall")]
    pub fn test_badge_size(size: BadgeSize) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Badge size=ResponsiveVec(vec![Responsive::Default(size), Responsive::Small(size), Responsive::Medium(size), Responsive::Large(size), Responsive::ExtraLarge(size), Responsive::ExtraExtraLarge(size)])>badge</Badge>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-badge-start--><!--leptos-view|components-src-data_display-badge.rs-70|open--><span class=\"badge{}{}{}{}{}{}\" id=\"_0-2\"><!--leptos-view|<Badge/>-children|open--><!--hk=_0-3o|leptos--start-->badge<!--hk=_0-3c|leptos--end--><!--leptos-view|<Badge/>-children|close--></span><!--leptos-view|components-src-data_display-badge.rs-70|close--><!--hk=_0-1c|leptos-badge-end-->",
                    if size == BadgeSize::Size(Size::Default) {"".to_string()} else {format!("{}", size)},
                    if size == BadgeSize::Size(Size::Default) {"".to_string()} else {format!(" sm:{}", size.class_name())},
                    if size == BadgeSize::Size(Size::Default) {"".to_string()} else {format!(" md:{}", size.class_name())},
                    if size == BadgeSize::Size(Size::Default) {"".to_string()} else {format!(" lg:{}", size.class_name())},
                    if size == BadgeSize::Size(Size::Default) {"".to_string()} else {format!(" xl:{}", size.class_name())},
                    if size == BadgeSize::Size(Size::Default) {"".to_string()} else {format!(" 2xl:{}", size.class_name())}
                )
            );
        }).dispose();
    }
}
