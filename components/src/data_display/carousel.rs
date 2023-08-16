use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CarouselSnap {
    Default,
    Start,
    Center,
    End,
}

impl ClassName for CarouselSnap {
    fn has_class_name(self) -> bool {
        self != CarouselSnap::Default && self != CarouselSnap::Start
    }

    fn class_name(self) -> String {
        match self {
            CarouselSnap::Default => "".to_string(),
            CarouselSnap::Start => "".to_string(),
            CarouselSnap::Center => "carousel-center".to_string(),
            CarouselSnap::End => "carousel-end".to_string(),
        }
    }
}

impl Display for CarouselSnap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CarouselOrientation {
    Default,
    Horizontal,
    Vertical,
}

impl ClassName for CarouselOrientation {
    fn has_class_name(self) -> bool {
        self != CarouselOrientation::Default && self != CarouselOrientation::Horizontal
    }

    fn class_name(self) -> String {
        match self {
            CarouselOrientation::Default => "".to_string(),
            CarouselOrientation::Horizontal => "".to_string(),
            CarouselOrientation::Vertical => "carousel-vertical".to_string(),
        }
    }
}

impl Display for CarouselOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[slot]
pub struct CarouselItem {
    pub children: ChildrenFn,
}

#[component]
pub fn Carousel(
    cx: Scope,
    id: String,
    #[prop(default = false)] indicator: bool,
    #[prop(default = false)] prev_next: bool,
    #[prop(default = CarouselSnap::Default)] snap: CarouselSnap,
    #[prop(default = CarouselOrientation::Default)] orientation: CarouselOrientation,
    #[prop(default=vec![])] carousel_item: Vec<CarouselItem>,
) -> impl IntoView {
    let indicator_controls = carousel_item
        .iter()
        .enumerate()
        .map(|(idx, _)| view! {cx, <a href=format!("#{}-item-{}", id, idx) class="btn btn-xs">{idx}</a>})
        .collect_view(cx);
    let items = carousel_item
        .iter()
        .enumerate()
        .map(|(idx, item)|view! {cx,
            <div class=format!("carousel-item{}", if prev_next {" relative"} else {""}) id=format!("{}-item-{}", id, idx)>
                {(item.children)(cx).into_view(cx)}
                if prev_next {(view!{cx,
                    <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                        <a href=format!("#{}-item-{}", id, idx) class="btn btn-circle">{"<"}</a>
                        <a href=format!("#{}-item-{}", id, idx) class="btn btn-circle">{">"}</a>
                    </div>
                }).into_view(cx)} else {().into_view(cx)}
            </div>
        });
    view! {cx,
        <div class=format!("carousel{}{}", snap, orientation)>
            {items.collect_view(cx)}
        </div>
        {if indicator {(view! {cx, <div class="flex justify-center w-full py-2 gap-2">{indicator_controls}</div>}).into_view(cx)} else {().into_view(cx)}}
    }
}
