use std::fmt::Display;

use leptos::html::Dialog;
use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ModalPosition {
    Default,
    Top,
    Middle,
    Bottom,
}

impl ClassName for ModalPosition {
    fn has_class_name(self) -> bool {
        self != ModalPosition::Default
    }

    fn class_name(self) -> String {
        String::from(match self {
            ModalPosition::Default => "",
            ModalPosition::Top => "modal-top",
            ModalPosition::Middle => "modal-middle",
            ModalPosition::Bottom => "modal-bottom",
        })
    }
}

impl Display for ModalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[slot]
pub struct ModalActions {
    children: ChildrenFn,
}

#[slot]
pub struct ModalEntrance {
    children: ChildrenFn,
}

#[component]
pub fn Modal(
    cx: Scope,
    modal_ref: NodeRef<Dialog>,
    #[prop(default = ModalPosition::Default)] position: ModalPosition,
    #[prop(optional)] modal_actions: Option<ModalActions>,
    #[prop(optional)] modal_entrance: Option<ModalEntrance>,
    children: Children,
) -> impl IntoView {
    view! {cx,
        {if let Some(modal_entrance) = &modal_entrance {(modal_entrance.children)(cx).into_view(cx)} else {().into_view(cx)}}
        <dialog class=format!("modal{}{}", if false {" modal-open"} else {""}, position) node_ref=modal_ref>
            <form method="dialog" class="modal-box">
                {children(cx)}
                {if let Some(modal_actions) = &modal_actions {(
                    view! {cx,
                        <div class="modal-action">
                            {(modal_actions.children)(cx)}
                        </div>
                    }
                ).into_view(cx)} else {().into_view(cx)}}

            </form>
            <form method="dialog" class="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    }
}
