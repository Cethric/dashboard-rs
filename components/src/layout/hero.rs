use leptos::*;

use crate::colour::Colour;
use crate::common::TextBackgroundColour;

pub enum HeroImageStyle {
    None(),
    Overlay(&'static str),
    FigureStart(&'static str),
    FigureEnd(&'static str),
}

#[component]
pub fn Hero(
    cx: Scope,
    #[prop(default=TextBackgroundColour::Colour(Colour::Default))] colour: TextBackgroundColour,
    #[prop(default = HeroImageStyle::None())] image: HeroImageStyle,
    children: Children,
) -> impl IntoView {
    let overlay = match image {
        HeroImageStyle::None() => false,
        HeroImageStyle::Overlay(_) => true,
        HeroImageStyle::FigureStart(_) => false,
        HeroImageStyle::FigureEnd(_) => false,
    };
    // todo implement images
    // let imageSrc = match image {
    //     HeroImageStyle::None() => "",
    //     HeroImageStyle::Overlay(src) => src,
    //     HeroImageStyle::FigureStart(src) => src,
    //     HeroImageStyle::FigureEnd(src) => src,
    // };

    view! {cx,
        <div class=format!("hero min-h-screen{}", colour)>
            {if overlay {(view! {cx, <div class="hero-overlay"/>}).into_view(cx)} else {().into_view(cx)}}
            <div class="hero-content">
                {children(cx)}
            </div>
        </div>
    }
}
