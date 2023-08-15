use std::fmt::Display;

use leptos::*;

use crate::class_name::{fmt_class_name, ClassName};
use crate::colour::Colour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChatSide {
    Start,
    End,
}

impl ClassName for ChatSide {
    fn has_class_name(self) -> bool {
        true
    }

    fn class_name(self) -> String {
        String::from(match self {
            ChatSide::Start => "chat-start",
            ChatSide::End => "chat-end",
        })
    }
}

impl Display for ChatSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChatColourBase<T> {
    Colour(T),
}

pub type ChatColour = ChatColourBase<Colour>;

impl ClassName for ChatColour {
    fn has_class_name(self) -> bool {
        self != ChatColour::Colour(Colour::Default)
    }

    fn class_name(self) -> String {
        String::from(match self {
            ChatColour::Colour(Colour::Default) => "".to_string(),
            ChatColour::Colour(color) => format!("chat-{}", color),
        })
    }
}

impl Display for ChatColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[slot]
pub struct ChatImage {
    children: ChildrenFn,
}

#[slot]
pub struct ChatHeader {
    children: ChildrenFn,
}

#[slot]
pub struct ChatFooter {
    children: ChildrenFn,
}

#[slot]
pub struct ChatBubble {
    children: ChildrenFn,
}

#[component]
pub fn Chat(
    cx: Scope,
    #[prop(default = ChatSide::Start)] side: ChatSide,
    #[prop(default = ChatColour::Colour(Colour::Default))] colour: ChatColour,
    #[prop(optional)] chat_image: Option<ChatImage>,
    #[prop(optional)] chat_header: Option<ChatHeader>,
    #[prop(optional)] chat_footer: Option<ChatFooter>,
    chat_bubble: ChatBubble,
) -> impl IntoView {
    view! {cx,
        <div class=format!("chat{}{}", side, colour)>
            {if let Some(chat_image) = chat_image {(view! {cx,
                <div class="chat-image avatar">
                    <div class="w-10 rounded-full">
                        {(chat_image.children)(cx)}
                    </div>
                </div>
            }).into_view(cx)} else {().into_view(cx)}}
            {if let Some(chat_header) = chat_header {(view! {cx,
                <div class="chat-header">
                    {(chat_header.children)(cx)}
                </div>
            }).into_view(cx)} else {().into_view(cx)}}
            <div class="chat-bubble">{(chat_bubble.children)(cx)}</div>
            {if let Some(chat_footer) = chat_footer {(view! {cx,
                <div class="chat-footer">
                    {(chat_footer.children)(cx)}
                </div>
            }).into_view(cx)} else {().into_view(cx)}}
        </div>
    }
}
