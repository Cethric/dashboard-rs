use leptos::*;

#[slot]
pub struct SwapOn {
    children: ChildrenFn,
}
#[slot]
pub struct SwapOff {
    children: ChildrenFn,
}
#[slot]
pub struct SwapIndeterminate {
    children: ChildrenFn,
}

#[component]
pub fn Swap(
    cx: Scope,
    #[prop(default = false)] rotate: bool,
    #[prop(default = false)] flip: bool,
    swap_on: SwapOn,
    swap_off: SwapOff,
    #[prop(optional)] swap_indeterminate: Option<SwapIndeterminate>,
) -> impl IntoView {
    view! {cx,
        <label class={
            format!(
                "swap{}{}",
                if rotate {" swap-rotate"} else {""},
                if flip {" swap-flip"} else {""}
            )
        }>
            <input type="checkbox"/>
            <div class="swap-on fill-current">{(swap_on.children)(cx).into_view(cx)}</div>
            {if let Some(swap_indeterminate) = &swap_indeterminate {(
                view! {cx,
                    <div class="swap-indeterminate fill-current">
                        {(swap_indeterminate.children)(cx)}
                    </div>
                }
            ).into_view(cx)} else {().into_view(cx)}}
            <div class="swap-off fill-current">{(swap_off.children)(cx).into_view(cx)}</div>
        </label>
    }
}
