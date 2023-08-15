#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    use components::data_display::collapse::*;

    #[test_case(CollapseIcon::Default; "Default")]
    #[test_case(CollapseIcon::Arrow; "Arrow")]
    #[test_case(CollapseIcon::Plus; "Plus")]
    pub fn test_button(icon: CollapseIcon) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <CollapseDetails icon=icon><CollapseTitle slot>title</CollapseTitle><CollapseContent slot>title</CollapseContent></CollapseDetails>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-collapse-details-start--><!--leptos-view|components-src-data_display-collapse.rs-71|open--><details class=\"collapse{}\" id=\"_0-2\"><summary class=\"collapse-title\" id=\"_0-3\"><!--leptos-view|<CollapseTitle/>-children|open--><!--hk=_0-4o|leptos--start-->title<!--hk=_0-4c|leptos--end--><!--leptos-view|<CollapseTitle/>-children|close--></summary><div class=\"collapse-content\" id=\"_0-5\"><!--leptos-view|<CollapseContent/>-children|open--><!--hk=_0-6o|leptos--start-->title<!--hk=_0-6c|leptos--end--><!--leptos-view|<CollapseContent/>-children|close--></div></details><!--leptos-view|components-src-data_display-collapse.rs-71|close--><!--hk=_0-1c|leptos-collapse-details-end-->",
                    icon
                )
            );
        }).dispose();
    }
}
