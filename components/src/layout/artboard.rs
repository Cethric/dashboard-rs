use std::fmt::{Display, Formatter};

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ArtBoardPhone {
    Phone(u8),
}

impl ClassName for ArtBoardPhone {
    fn has_class_name(self) -> bool {
        match self {
            ArtBoardPhone::Phone(phone) => {
                assert!(phone < 7);
                phone > 0
            }
        }
    }

    fn class_name(self) -> String {
        match self {
            ArtBoardPhone::Phone(phone) => {
                assert!(phone < 7);
                format!("phone-{:#}", phone)
            }
        }
    }
}

impl Display for ArtBoardPhone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn ArtBoard(
    cx: Scope,
    #[prop(default = false)] demo: bool,
    #[prop(default = false)] horizontal: bool,
    #[prop(default = ArtBoardPhone::Phone(1))] phone: ArtBoardPhone,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class=format!("artboard{}{}{}", if demo {" artboard-demo"} else {""}, if horizontal {" artboard-horizontal"} else {""}, phone)>{children(cx)}</div>
    }
}
