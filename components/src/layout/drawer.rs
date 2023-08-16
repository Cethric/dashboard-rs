use leptos::*;

#[slot]
pub struct DrawerPageContent {
    children: ChildrenFn,
}
#[slot]
pub struct DrawerContent {
    children: ChildrenFn,
}

#[component]
pub fn Drawer(
    cx: Scope,
    drawer_page_content: DrawerPageContent,
    drawer_content: DrawerContent,
) -> impl IntoView {
    view! {cx,
        <div class="drawer">
            <input class="drawer-toggle" type="checkbox"/>
            <div class="drawer-content">
                {(drawer_page_content.children)(cx)}
            </div>
            <div class="drawer-side">
                <label for="" class="drawer-overlay"></label>
                {(drawer_content.children)(cx)}
            </div>
        </div>
    }
}
