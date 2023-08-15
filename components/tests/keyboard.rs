#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    use components::class_name::ClassName;
    use components::data_display::keyboard::*;
    use components::responsive::{Responsive, ResponsiveVec};
    use components::size::Size;

    #[test_case(KeyboardSize::Size(Size::Default); "Default")]
    #[test_case(KeyboardSize::Size(Size::Large); "Large")]
    #[test_case(KeyboardSize::Size(Size::Medium); "Medium")]
    #[test_case(KeyboardSize::Size(Size::Small); "Small")]
    #[test_case(KeyboardSize::Size(Size::ExtraSmall); "ExtraSmall")]
    pub fn test_keyboard_size(size: KeyboardSize) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Keyboard size=ResponsiveVec(vec![Responsive::Default(size), Responsive::Small(size), Responsive::Medium(size), Responsive::Large(size), Responsive::ExtraLarge(size), Responsive::ExtraExtraLarge(size)])>F</Keyboard>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-keyboard-start--><!--leptos-view|components-src-data_display-keyboard.rs-41|open--><kbd class=\"kbd{}{}{}{}{}{}\" id=\"_0-2\"><!--leptos-view|<Keyboard/>-children|open--><!--hk=_0-3o|leptos--start-->F<!--hk=_0-3c|leptos--end--><!--leptos-view|<Keyboard/>-children|close--></kbd><!--leptos-view|components-src-data_display-keyboard.rs-41|close--><!--hk=_0-1c|leptos-keyboard-end-->",
                    if size == KeyboardSize::Size(Size::Default) {"".to_string()} else {format!("{}", size)},
                    if size == KeyboardSize::Size(Size::Default) {"".to_string()} else {format!(" sm:{}", size.class_name())},
                    if size == KeyboardSize::Size(Size::Default) {"".to_string()} else {format!(" md:{}", size.class_name())},
                    if size == KeyboardSize::Size(Size::Default) {"".to_string()} else {format!(" lg:{}", size.class_name())},
                    if size == KeyboardSize::Size(Size::Default) {"".to_string()} else {format!(" xl:{}", size.class_name())},
                    if size == KeyboardSize::Size(Size::Default) {"".to_string()} else {format!(" 2xl:{}", size.class_name())}
                )
            );
        }).dispose();
    }
}
