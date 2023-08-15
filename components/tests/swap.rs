#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    use components::actions::swap::*;

    #[test_case(false, false; "no style")]
    #[test_case(true, false; "rotate on")]
    #[test_case(false, true; "flip on")]
    #[test_case(true, true; "rotate & flip on")]
    pub fn test_swap(rotate: bool, flip: bool) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Swap rotate=rotate flip=flip><SwapOn slot>On</SwapOn><SwapOff slot>Off</SwapOff></Swap>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-swap-start--><!--leptos-view|components-src-actions-swap.rs-25|open--><label class=\"swap{}{}\" id=\"_0-2\"><input type=\"checkbox\" id=\"_0-3\"/><div class=\"swap-on fill-current\" id=\"_0-4\"><!--leptos-view|<SwapOn/>-children|open--><!--hk=_0-5o|leptos--start-->On<!--hk=_0-5c|leptos--end--><!--leptos-view|<SwapOn/>-children|close--></div><!--hk=_0-6c|leptos-unit--><div class=\"swap-off fill-current\" id=\"_0-7\"><!--leptos-view|<SwapOff/>-children|open--><!--hk=_0-8o|leptos--start-->Off<!--hk=_0-8c|leptos--end--><!--leptos-view|<SwapOff/>-children|close--></div></label><!--leptos-view|components-src-actions-swap.rs-25|close--><!--hk=_0-1c|leptos-swap-end-->",
                    if rotate {" swap-rotate"} else {""},
                    if flip {" swap-flip"} else {""}
                )
            );
        }).dispose();
    }

    #[test]
    pub fn test_swap_indeterminate() {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Swap><SwapOn slot>On</SwapOn><SwapOff slot>Off</SwapOff><SwapIndeterminate slot>Indeterminate</SwapIndeterminate></Swap>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-swap-start--><!--leptos-view|components-src-actions-swap.rs-25|open--><label class=\"swap\" id=\"_0-2\"><input type=\"checkbox\" id=\"_0-3\"/><div class=\"swap-on fill-current\" id=\"_0-4\"><!--leptos-view|<SwapOn/>-children|open--><!--hk=_0-5o|leptos--start-->On<!--hk=_0-5c|leptos--end--><!--leptos-view|<SwapOn/>-children|close--></div><!--leptos-view|components-src-actions-swap.rs-36|open--><div class=\"swap-indeterminate fill-current\" id=\"_0-6\"><!--leptos-view|<SwapIndeterminate/>-children|open--><!--hk=_0-7o|leptos--start-->Indeterminate<!--hk=_0-7c|leptos--end--><!--leptos-view|<SwapIndeterminate/>-children|close--></div><!--leptos-view|components-src-actions-swap.rs-36|close--><div class=\"swap-off fill-current\" id=\"_0-8\"><!--leptos-view|<SwapOff/>-children|open--><!--hk=_0-9o|leptos--start-->Off<!--hk=_0-9c|leptos--end--><!--leptos-view|<SwapOff/>-children|close--></div></label><!--leptos-view|components-src-actions-swap.rs-25|close--><!--hk=_0-1c|leptos-swap-end-->",
                )
            );
        }).dispose();
    }
}
