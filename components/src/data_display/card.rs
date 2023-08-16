use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::responsive::{Responsive, ResponsiveVec};

#[slot]
pub struct CardTitle {
    pub children: ChildrenFn,
}

#[slot]
pub struct CardBody {
    pub children: ChildrenFn,
}

#[slot]
pub struct CardActions {
    pub children: ChildrenFn,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CardImagePosition {
    Default,
    Top,
    Full,
    Bottom,
}

#[slot]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CardImage {
    pub position: CardImagePosition,
    pub children: ChildrenFn,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CardPadding {
    Normal,
    Compact,
}

impl ClassName for CardPadding {
    fn has_class_name(self) -> bool {
        true
    }

    fn class_name(self) -> String {
        match self {
            CardPadding::Normal => "card-normal".to_string(),
            CardPadding::Compact => "card-compact".to_string(),
        }
    }
}

impl Display for CardPadding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CardSide {
    Side,
}

impl ClassName for CardSide {
    fn has_class_name(self) -> bool {
        true
    }

    fn class_name(self) -> String {
        match self {
            CardSide::Side => "card-side".to_string(),
        }
    }
}

impl Display for CardSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

fn check_card_image_position_eq(
    card_image: &Option<CardImage>,
    position: CardImagePosition,
) -> bool {
    if let Some(ref card_image) = card_image {
        return card_image.position == position;
    }
    false
}
fn check_card_image_position_ne(
    card_image: &Option<CardImage>,
    position: CardImagePosition,
) -> bool {
    if let Some(ref card_image) = card_image {
        return card_image.position != position;
    }
    false
}

#[component]
pub fn Card(
    cx: Scope,
    card_title: CardTitle,
    card_body: CardBody,
    card_actions: CardActions,
    #[prop(optional)] card_image: Option<CardImage>,
    #[prop(default = false)] bordered: bool,
    #[prop(default=ResponsiveVec(vec![]))] padding: ResponsiveVec<Responsive<CardPadding>>,
    #[prop(default=ResponsiveVec(vec![]))] side: ResponsiveVec<Responsive<CardSide>>,
) -> impl IntoView {
    let image = || {
        if let Some(ref card_image) = card_image {
            (view! {cx, <figure>{(card_image.children)(cx)}</figure>}).into_view(cx)
        } else {
            ().into_view(cx)
        }
    };

    view! { cx,
        <div
            class={
                format!(
                    "card{}{}{}{} w-96 bg-base-100 shadow-xl",
                    if check_card_image_position_eq(&card_image, CardImagePosition::Full) {" image-full"} else {""},
                    side,
                    if bordered {" card-bordered"} else {""},
                    padding
                )
            }
        >
            {if check_card_image_position_ne(&card_image, CardImagePosition::Bottom) {image().into_view(cx)} else {().into_view(cx)}}
            <div class="card-body">
                <h2 class="card-title">{(card_title.children)(cx)}</h2>
                {(card_body.children)(cx)}
                <div class="card-actions justify-end">
                    {(card_actions.children)(cx)}
                </div>
            </div>
            {if check_card_image_position_eq(&card_image, CardImagePosition::Bottom) {image().into_view(cx)} else {().into_view(cx)}}
        </div>
    }
}
