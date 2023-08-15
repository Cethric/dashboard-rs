#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    use components::actions::button::*;
    use components::class_name::ClassName;
    #[allow(unused_imports)]
    use components::colour::Colour;
    use components::responsive::{Responsive, ResponsiveVec};
    use components::size::Size;

    #[test_case(false, false, false, false, false, false, false; "default")]
    #[test_case(false, false, false, false, false, false, true; "glass")]
    #[test_case(false, false, false, false, false, true, false; "no_animation")]
    #[test_case(false, false, false, false, true, false, false; "disabled")]
    #[test_case(false, false, false, true, false, false, false; "block")]
    #[test_case(false, false, true, false, false, false, false; "wide")]
    #[test_case(false, true, false, false, false, false, false; "outline")]
    #[test_case(true, false, false, false, false, false, false; "active")]
    pub fn test_button(
        active: bool,
        outline: bool,
        wide: bool,
        block: bool,
        disabled: bool,
        no_animation: bool,
        glass: bool,
    ) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Button active=active outline=outline wide=wide block=block disabled=disabled no_animation=no_animation glass=glass>button</Button>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-button-start--><!--leptos-view|components-src-actions-button.rs-132|open--><button{} class=\"btn{}{}{}{}{}{}{}\" type id=\"_0-2\"><!--leptos-view|<Button/>-children|open--><!--hk=_0-3o|leptos--start-->button<!--hk=_0-3c|leptos--end--><!--leptos-view|<Button/>-children|close--></button><!--leptos-view|components-src-actions-button.rs-132|close--><!--hk=_0-1c|leptos-button-end-->",
                    if disabled { " disabled" } else { "" },
                    if active { " btn-active" } else { "" },
                    if outline { " btn-outline" } else { "" },
                    if wide { " btn-wide" } else { "" },
                    if block { " btn-block" } else { "" },
                    if disabled { " btn-disabled" } else { "" },
                    if no_animation { " no-animation" } else { "" },
                    if glass { " glass" } else { "" }
                )
            );
        }).dispose();
    }

    #[test_case(ButtonColour::Colour(Colour::Default); "Default")]
    #[test_case(ButtonColour::Colour(Colour::Neutral); "Neutral")]
    #[test_case(ButtonColour::Colour(Colour::Primary); "Primary")]
    #[test_case(ButtonColour::Colour(Colour::Secondary); "Secondary")]
    #[test_case(ButtonColour::Colour(Colour::Accent); "Accent")]
    #[test_case(ButtonColour::Ghost(); "Ghost")]
    #[test_case(ButtonColour::Link(); "Link")]
    #[test_case(ButtonColour::Colour(Colour::Info); "Info")]
    #[test_case(ButtonColour::Colour(Colour::Success); "Success")]
    #[test_case(ButtonColour::Colour(Colour::Warning); "Warning")]
    #[test_case(ButtonColour::Colour(Colour::Error); "Error")]
    pub fn test_button_colour(colour: ButtonColour) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Button colour=colour>button</Button>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-button-start--><!--leptos-view|components-src-actions-button.rs-132|open--><button class=\"btn{}\" type id=\"_0-2\"><!--leptos-view|<Button/>-children|open--><!--hk=_0-3o|leptos--start-->button<!--hk=_0-3c|leptos--end--><!--leptos-view|<Button/>-children|close--></button><!--leptos-view|components-src-actions-button.rs-132|close--><!--hk=_0-1c|leptos-button-end-->",
                    colour
                )
            );
        }).dispose();
    }

    #[test_case(ButtonSize::Size(Size::Default); "Default")]
    #[test_case(ButtonSize::Size(Size::Large); "Large")]
    #[test_case(ButtonSize::Size(Size::Medium); "Medium")]
    #[test_case(ButtonSize::Size(Size::Small); "Small")]
    #[test_case(ButtonSize::Size(Size::ExtraSmall); "ExtraSmall")]
    pub fn test_button_size(size: ButtonSize) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Button size=ResponsiveVec(vec![Responsive::Default(size), Responsive::Small(size), Responsive::Medium(size), Responsive::Large(size), Responsive::ExtraLarge(size), Responsive::ExtraExtraLarge(size)])>button</Button>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-button-start--><!--leptos-view|components-src-actions-button.rs-132|open--><button class=\"btn{}{}{}{}{}{}\" type id=\"_0-2\"><!--leptos-view|<Button/>-children|open--><!--hk=_0-3o|leptos--start-->button<!--hk=_0-3c|leptos--end--><!--leptos-view|<Button/>-children|close--></button><!--leptos-view|components-src-actions-button.rs-132|close--><!--hk=_0-1c|leptos-button-end-->",
                    if size == ButtonSize::Size(Size::Default) {"".to_string()} else {format!("{}", size)},
                    if size == ButtonSize::Size(Size::Default) {"".to_string()} else {format!(" sm:{}", size.class_name())},
                    if size == ButtonSize::Size(Size::Default) {"".to_string()} else {format!(" md:{}", size.class_name())},
                    if size == ButtonSize::Size(Size::Default) {"".to_string()} else {format!(" lg:{}", size.class_name())},
                    if size == ButtonSize::Size(Size::Default) {"".to_string()} else {format!(" xl:{}", size.class_name())},
                    if size == ButtonSize::Size(Size::Default) {"".to_string()} else {format!(" 2xl:{}", size.class_name())}
                )
            );
        }).dispose();
    }

    #[test_case(ButtonShape::Square; "Square")]
    #[test_case(ButtonShape::Circle; "Circle")]
    pub fn test_button_shape(shape: ButtonShape) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Button shape=ResponsiveVec(vec![Responsive::Default(shape), Responsive::Small(shape), Responsive::Medium(shape), Responsive::Large(shape), Responsive::ExtraLarge(shape), Responsive::ExtraExtraLarge(shape)])>button</Button>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-button-start--><!--leptos-view|components-src-actions-button.rs-132|open--><button class=\"btn{}{}{}{}{}{}\" type id=\"_0-2\"><!--leptos-view|<Button/>-children|open--><!--hk=_0-3o|leptos--start-->button<!--hk=_0-3c|leptos--end--><!--leptos-view|<Button/>-children|close--></button><!--leptos-view|components-src-actions-button.rs-132|close--><!--hk=_0-1c|leptos-button-end-->",
                    shape,
                    format!(" sm:{}", shape.class_name()),
                    format!(" md:{}", shape.class_name()),
                    format!(" lg:{}", shape.class_name()),
                    format!(" xl:{}", shape.class_name()),
                    format!(" 2xl:{}", shape.class_name())
                )
            );
        }).dispose();
    }
}
