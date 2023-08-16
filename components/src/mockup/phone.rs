use leptos::*;

use crate::layout::artboard::{Artboard, ArtboardPhone};

#[component]
pub fn MockupPhone(
    cx: Scope,
    #[prop(default = ArtboardPhone::Phone(1))] phone: ArtboardPhone,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class="mockup-phone">
            <div class="camera"/>
            <div class="display">
                <Artboard phone=phone>
                    {children(cx)}
                </Artboard>
            </div>
        </div>
    }
}
