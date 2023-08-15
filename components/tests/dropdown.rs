#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    use components::actions::dropdown::*;

    #[test_case(false; "hover off")]
    #[test_case(true; "hover on")]
    pub fn test_dropdown_hover(hover: bool) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Dropdown label="abc" hover=hover><li><ul>item</ul></li></Dropdown>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-dropdown-start--><!--leptos-view|components-src-actions-dropdown.rs-83|open--><details class=\"dropdown{}\" id=\"_0-2\"><summary class=\"btn\" id=\"_0-3\">abc</summary><!--hk=_0-4o|leptos-dropdown-content-start--><!--leptos-view|components-src-actions-dropdown.rs-67|open--><div class=\"dropdown-content shadow z-[1] bg-base-300 rounded-box w-52\" id=\"_0-5\"><!--leptos-view|<DropdownContent/>-children|open--><!--hk=_0-6o|leptos--start--><!--leptos-view|<Dropdown/>-children|open--><!--hk=_0-7o|leptos--start--><li id=\"_0-8\"><ul id=\"_0-9\">item</ul></li><!--hk=_0-7c|leptos--end--><!--leptos-view|<Dropdown/>-children|close--><!--hk=_0-6c|leptos--end--><!--leptos-view|<DropdownContent/>-children|close--></div><!--leptos-view|components-src-actions-dropdown.rs-67|close--><!--hk=_0-4c|leptos-dropdown-content-end--></details><!--leptos-view|components-src-actions-dropdown.rs-83|close--><!--hk=_0-1c|leptos-dropdown-end-->",
                    if hover {" dropdown-hover"} else {""}
                )
            );
        }).dispose();
    }

    #[test_case(DropdownPosition::Default; "Default")]
    #[test_case(DropdownPosition::Top; "Top")]
    #[test_case(DropdownPosition::Bottom; "Bottom")]
    #[test_case(DropdownPosition::Left; "Left")]
    #[test_case(DropdownPosition::Right; "Right")]
    pub fn test_dropdown_position(position: DropdownPosition) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Dropdown label="abc" position=position><li><ul>item</ul></li></Dropdown>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-dropdown-start--><!--leptos-view|components-src-actions-dropdown.rs-83|open--><details class=\"dropdown{}\" id=\"_0-2\"><summary class=\"btn\" id=\"_0-3\">abc</summary><!--hk=_0-4o|leptos-dropdown-content-start--><!--leptos-view|components-src-actions-dropdown.rs-67|open--><div class=\"dropdown-content shadow z-[1] bg-base-300 rounded-box w-52\" id=\"_0-5\"><!--leptos-view|<DropdownContent/>-children|open--><!--hk=_0-6o|leptos--start--><!--leptos-view|<Dropdown/>-children|open--><!--hk=_0-7o|leptos--start--><li id=\"_0-8\"><ul id=\"_0-9\">item</ul></li><!--hk=_0-7c|leptos--end--><!--leptos-view|<Dropdown/>-children|close--><!--hk=_0-6c|leptos--end--><!--leptos-view|<DropdownContent/>-children|close--></div><!--leptos-view|components-src-actions-dropdown.rs-67|close--><!--hk=_0-4c|leptos-dropdown-content-end--></details><!--leptos-view|components-src-actions-dropdown.rs-83|close--><!--hk=_0-1c|leptos-dropdown-end-->",
                    position
                )
            );
        }).dispose();
    }
    #[test_case(DropdownAlign::Default; "Default")]
    #[test_case(DropdownAlign::Start; "Start")]
    #[test_case(DropdownAlign::End; "End")]
    pub fn test_dropdown_align(align: DropdownAlign) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Dropdown label="abc" align=align><li><ul>item</ul></li></Dropdown>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-dropdown-start--><!--leptos-view|components-src-actions-dropdown.rs-83|open--><details class=\"dropdown{}\" id=\"_0-2\"><summary class=\"btn\" id=\"_0-3\">abc</summary><!--hk=_0-4o|leptos-dropdown-content-start--><!--leptos-view|components-src-actions-dropdown.rs-67|open--><div class=\"dropdown-content shadow z-[1] bg-base-300 rounded-box w-52\" id=\"_0-5\"><!--leptos-view|<DropdownContent/>-children|open--><!--hk=_0-6o|leptos--start--><!--leptos-view|<Dropdown/>-children|open--><!--hk=_0-7o|leptos--start--><li id=\"_0-8\"><ul id=\"_0-9\">item</ul></li><!--hk=_0-7c|leptos--end--><!--leptos-view|<Dropdown/>-children|close--><!--hk=_0-6c|leptos--end--><!--leptos-view|<DropdownContent/>-children|close--></div><!--leptos-view|components-src-actions-dropdown.rs-67|close--><!--hk=_0-4c|leptos-dropdown-content-end--></details><!--leptos-view|components-src-actions-dropdown.rs-83|close--><!--hk=_0-1c|leptos-dropdown-end-->",
                    align
                )
            );
        }).dispose();
    }
}
