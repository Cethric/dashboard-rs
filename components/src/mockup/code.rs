use leptos::*;

use crate::colour::Colour;
use crate::common::TextBackgroundColour;

#[slot]
pub struct MockupCodeLine {
    children: ChildrenFn,
    #[prop(default = TextBackgroundColour::Colour(Colour::Default))]
    colour: TextBackgroundColour,
    line: Option<u64>,
}

#[component]
pub fn MockupCode(
    cx: Scope,
    #[prop(default = TextBackgroundColour::Colour(Colour::Default))] colour: TextBackgroundColour,
    mockup_code_line: Vec<MockupCodeLine>,
) -> impl IntoView {
    view! {cx,
        <div class=format!("mockup-code{}", colour)>
            {
                mockup_code_line
                    .iter()
                    .map(|item| match item.line {
                        Some(line)=> view! {cx, <pre data-prefix=line class=format!("{}", item.colour)>{(item.children)(cx)}</pre>},
                        None => view! {cx, <pre class=format!("{}", item.colour)><code>{(item.children)(cx)}</code></pre>}
                    })
                    .collect_view(cx)
            }
        </div>
    }
}
