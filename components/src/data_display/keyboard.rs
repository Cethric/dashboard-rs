use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::responsive::{Responsive, ResponsiveVec};
use crate::size::Size;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardSizeBase<T> {
    Size(T),
}

pub type KeyboardSize = KeyboardSizeBase<Size>;

impl ClassName for KeyboardSize {
    fn has_class_name(self) -> bool {
        self != KeyboardSize::Size(Size::Default)
    }

    fn class_name(self) -> String {
        match self {
            KeyboardSize::Size(Size::Default) => "".to_string(),
            KeyboardSize::Size(size) => format!("kbd-{}", size),
        }
    }
}

impl Display for KeyboardSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Keyboard(
    cx: Scope,
    #[prop(default=ResponsiveVec(vec![]))] size: ResponsiveVec<Responsive<KeyboardSize>>,
    children: Children,
) -> impl IntoView {
    view! {cx, <kbd class=format!("kbd{}", size)>{children(cx)}</kbd>}
}
