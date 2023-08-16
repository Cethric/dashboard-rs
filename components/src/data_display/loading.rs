use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;
use crate::responsive::{Responsive, ResponsiveVec};
use crate::size::Size;
use crate::typography::TextColour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LoadingVariant {
    Default,
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}

impl ClassName for LoadingVariant {
    fn has_class_name(self) -> bool {
        self != LoadingVariant::Default
    }

    fn class_name(self) -> String {
        match self {
            LoadingVariant::Default => "".to_string(),
            LoadingVariant::Spinner => "loading-spinner".to_string(),
            LoadingVariant::Dots => "loading-dots".to_string(),
            LoadingVariant::Ring => "loading-ring".to_string(),
            LoadingVariant::Ball => "loading-ball".to_string(),
            LoadingVariant::Bars => "loading-bars".to_string(),
            LoadingVariant::Infinity => "loading-infinity".to_string(),
        }
    }
}

impl Display for LoadingVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LoadingSizeBase<T> {
    Size(T),
}

pub type LoadingSize = LoadingSizeBase<Size>;

impl ClassName for LoadingSize {
    fn has_class_name(self) -> bool {
        self != LoadingSize::Size(Size::Default)
    }

    fn class_name(self) -> String {
        match self {
            LoadingSize::Size(Size::Default) => "".to_string(),
            LoadingSize::Size(size) => format!("loading-{}", size),
        }
    }
}

impl Display for LoadingSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[component]
pub fn Loading(
    cx: Scope,
    #[prop(default=TextColour::Colour(Colour::Default))] colour: TextColour,
    #[prop(default=LoadingVariant::Default)] variant: LoadingVariant,
    #[prop(default=ResponsiveVec(vec![]))] size: ResponsiveVec<Responsive<LoadingSize>>,
) -> impl IntoView {
    view! {cx, <span class=format!("loading{}{}{}", variant, size, colour)/>}
}
