#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    #[allow(unused_imports)]
    use components::colour::Colour;
    use components::data_display::chat::*;

    #[test_case(ChatColour::Colour(Colour::Default); "Default")]
    #[test_case(ChatColour::Colour(Colour::Neutral); "Neutral")]
    #[test_case(ChatColour::Colour(Colour::Primary); "Primary")]
    #[test_case(ChatColour::Colour(Colour::Secondary); "Secondary")]
    #[test_case(ChatColour::Colour(Colour::Accent); "Accent")]
    #[test_case(ChatColour::Colour(Colour::Info); "Info")]
    #[test_case(ChatColour::Colour(Colour::Success); "Success")]
    #[test_case(ChatColour::Colour(Colour::Warning); "Warning")]
    #[test_case(ChatColour::Colour(Colour::Error); "Error")]
    pub fn test_chat_variant(colour: ChatColour) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Chat colour=colour><ChatBubble slot>Hello</ChatBubble></Chat>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-chat-start--><!--leptos-view|components-src-data_display-chat.rs-89|open--><div class=\"chat chat-start{}\" id=\"_0-2\"><!--hk=_0-3c|leptos-unit--><!--hk=_0-4c|leptos-unit--><div class=\"chat-bubble\" id=\"_0-5\"><!--leptos-view|<ChatBubble/>-children|open--><!--hk=_0-6o|leptos--start-->Hello<!--hk=_0-6c|leptos--end--><!--leptos-view|<ChatBubble/>-children|close--></div><!--hk=_0-7c|leptos-unit--></div><!--leptos-view|components-src-data_display-chat.rs-89|close--><!--hk=_0-1c|leptos-chat-end-->",
                    colour
                )
            );
        }).dispose();
    }

    #[test_case(ChatSide::Start; "Start")]
    #[test_case(ChatSide::End; "End")]
    pub fn test_chat_side(side: ChatSide) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx, <Chat side=side><ChatBubble slot>Hello</ChatBubble></Chat>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-chat-start--><!--leptos-view|components-src-data_display-chat.rs-89|open--><div class=\"chat{}\" id=\"_0-2\"><!--hk=_0-3c|leptos-unit--><!--hk=_0-4c|leptos-unit--><div class=\"chat-bubble\" id=\"_0-5\"><!--leptos-view|<ChatBubble/>-children|open--><!--hk=_0-6o|leptos--start-->Hello<!--hk=_0-6c|leptos--end--><!--leptos-view|<ChatBubble/>-children|close--></div><!--hk=_0-7c|leptos-unit--></div><!--leptos-view|components-src-data_display-chat.rs-89|close--><!--hk=_0-1c|leptos-chat-end-->",
                    side
                )
            );
        }).dispose();
    }
}
