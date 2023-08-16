use std::fmt::{Display, Formatter};

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ArtboardPhone {
    Phone(u8),
}

impl ClassName for ArtboardPhone {
    fn has_class_name(self) -> bool {
        match self {
            ArtboardPhone::Phone(phone) => {
                assert!(phone < 7);
                phone > 0
            }
        }
    }

    fn class_name(self) -> String {
        match self {
            ArtboardPhone::Phone(phone) => {
                assert!(phone < 7);
                format!("phone-{:#}", phone)
            }
        }
    }
}

impl Display for ArtboardPhone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Artboard(
    cx: Scope,
    #[prop(default = false)] demo: bool,
    #[prop(default = false)] horizontal: bool,
    #[prop(default = ArtboardPhone::Phone(1))] phone: ArtboardPhone,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class=format!("artboard{}{}{}", if demo {" artboard-demo"} else {""}, if horizontal {" artboard-horizontal"} else {""}, phone)>{children(cx)}</div>
    }
}
