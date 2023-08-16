use leptos::*;

use crate::layout::artboard::{ArtBoard, ArtBoardPhone};

#[component]
pub fn MockupPhone(
    cx: Scope,
    #[prop(default = ArtBoardPhone::Phone(1))] phone: ArtBoardPhone,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class="mockup-phone">
            <div class="camera"/>
            <div class="display">
                <ArtBoard phone=phone>
                    {children(cx)}
                </ArtBoard>
            </div>
        </div>
    }
}
