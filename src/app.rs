use leptos::html::Dialog;
use leptos::*;
use leptos_icons::{BsIcon, Icon};
use log::info;
use wasm_bindgen::prelude::*;

use components::actions::button::*;
use components::actions::dropdown::*;
use components::actions::modal::*;
use components::actions::swap::*;
use components::colour::*;
use components::data_display::alert::*;
use components::data_display::badge::*;
use components::data_display::collapse::*;
use components::responsive::*;
use components::size::Size;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn ButtonExamples(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <Button>Default</Button>
            <Button colour=ButtonColour::Colour(Colour::Neutral)>Neutral</Button>
            <Button colour=ButtonColour::Colour(Colour::Primary)>Primary</Button>
            <Button colour=ButtonColour::Colour(Colour::Secondary)>Secondary</Button>
            <Button colour=ButtonColour::Colour(Colour::Accent)>Accent</Button>
            <Button colour=ButtonColour::Ghost()>Ghost</Button>
            <Button colour=ButtonColour::Link()>Link</Button>
        </div>
        <div>
            <Button active=true>Default Active</Button>
            <Button active=true colour=ButtonColour::Colour(Colour::Neutral)>Neutral Active</Button>
            <Button active=true colour=ButtonColour::Colour(Colour::Primary)>Primary Active</Button>
            <Button active=true colour=ButtonColour::Colour(Colour::Secondary)>Secondary Active</Button>
            <Button active=true colour=ButtonColour::Colour(Colour::Accent)>Accent Active</Button>
            <Button active=true colour=ButtonColour::Ghost()>Ghost Active</Button>
            <Button active=true colour=ButtonColour::Link()>Link Active</Button>
        </div>
        <div>
            <Button colour=ButtonColour::Colour(Colour::Info)>Info</Button>
            <Button colour=ButtonColour::Colour(Colour::Success)>Success</Button>
            <Button colour=ButtonColour::Colour(Colour::Warning)>Warning</Button>
            <Button colour=ButtonColour::Colour(Colour::Error)>Error</Button>
        </div>
        <div>
            <Button outline=true>Default</Button>
            <Button outline=true colour=ButtonColour::Colour(Colour::Primary)>Primary</Button>
            <Button outline=true colour=ButtonColour::Colour(Colour::Secondary)>Secondary</Button>
            <Button outline=true colour=ButtonColour::Colour(Colour::Accent)>Accent</Button>
        </div>
        <div>
            <Button outline=true colour=ButtonColour::Colour(Colour::Info)>Info</Button>
            <Button outline=true colour=ButtonColour::Colour(Colour::Success)>Success</Button>
            <Button outline=true colour=ButtonColour::Colour(Colour::Warning)>Warning</Button>
            <Button outline=true colour=ButtonColour::Colour(Colour::Error)>Error</Button>
        </div>
        <div>
            <Button size=ResponsiveVec(vec![Responsive::Default(ButtonSize::Size(Size::Large))])>Large</Button>
            <Button >Default</Button>
            <Button size=ResponsiveVec(vec![Responsive::Default(ButtonSize::Size(Size::Medium))])>Medium</Button>
            <Button size=ResponsiveVec(vec![Responsive::Default(ButtonSize::Size(Size::Small))])>Small</Button>
            <Button size=ResponsiveVec(vec![Responsive::Default(ButtonSize::Size(Size::ExtraSmall))])>ExtraSmall</Button>
        </div>
        <div>
            <Button wide=true>Default</Button>
        </div>
        <div>
            <Button disabled=true>Default</Button>
        </div>
        <div>
            <Button shape=ResponsiveVec(vec![Responsive::Default(ButtonShape::Square)])>X</Button>
            <Button shape=ResponsiveVec(vec![Responsive::Default(ButtonShape::Square)]) outline=true>X</Button>
        </div>
        <div>
            <Button shape=ResponsiveVec(vec![Responsive::Default(ButtonShape::Circle)])>X</Button>
            <Button shape=ResponsiveVec(vec![Responsive::Default(ButtonShape::Circle)]) outline=true>X</Button>
        </div>
        <div>
            <Button shape=ResponsiveVec(vec![Responsive::Large(ButtonShape::Circle), Responsive::Small(ButtonShape::Square)])>X</Button>
        </div>
        <div>
            <Button no_animation=true>I dont have click animation</Button>
        </div>
        <div>
            <Button glass=true>I have a glass effect</Button>
        </div>
    }
}

#[component]
pub fn DropdownExamples(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <Dropdown label="Open or Close">
                <ul class="menu p-2">
                    <li><a>Item 1</a></li>
                    <li><a>Item 2</a></li>
                    <li><a>Item 3</a></li>
                </ul>
            </Dropdown>
        </div>
    }
}

#[component]
pub fn ModalExamples(cx: Scope) -> impl IntoView {
    let modal_ref = create_node_ref::<Dialog>(cx);

    view! {cx,
        <div>
            <Modal position=ModalPosition::Middle modal_ref=modal_ref>
                <h3 class="font-bold text-lg">Hello!</h3>
                <p class="py-4">Press ESC key or click the button below to close</p>
                <ModalActions slot>
                    <Button button_type=ButtonType::Submit>Submit</Button>
                    <Button on:click=move |_| {info!("Close Modal"); modal_ref.get().expect("Invalid dialog").close();}>Close</Button>
                </ModalActions>
                <ModalEntrance slot>
                    <Button on:click=move |_| {info!("Open Modal"); let _ = modal_ref.get().expect("Invalid dialog").show_modal();}>Open Modal</Button>
                </ModalEntrance>
            </Modal>
        </div>
    }
}

#[component]
pub fn SwapExamples(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <Swap flip=true rotate=true>
                <SwapOn slot><Icon icon=Icon::Bs(BsIcon::BsGithub)/></SwapOn>
                <SwapOff slot><Icon icon=Icon::Bs(BsIcon::BsPower)/></SwapOff>
            </Swap>
        </div>
    }
}

#[component]
pub fn CollapseExamples(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <CollapseDetails>
                <CollapseTitle slot>Hello World</CollapseTitle>
                <CollapseContent slot>This is an example details collapse</CollapseContent>
            </CollapseDetails>
            <CollapseDetails icon=CollapseIcon::Arrow>
                <CollapseTitle slot>Hello World Arrow</CollapseTitle>
                <CollapseContent slot>This is an example details collapse</CollapseContent>
            </CollapseDetails>
            <CollapseDetails icon=CollapseIcon::Plus>
                <CollapseTitle slot>Hello World Plus</CollapseTitle>
                <CollapseContent slot>This is an example details collapse</CollapseContent>
            </CollapseDetails>
            <Collapse>
                <CollapseTitle slot>Open on focus</CollapseTitle>
                <CollapseContent slot>This is an example focus collapse</CollapseContent>
            </Collapse>
            <Collapse>
                <CollapseControl slot><input type="checkbox" /></CollapseControl>
                <CollapseTitle slot>Open on input</CollapseTitle>
                <CollapseContent slot>This is an example input collapse</CollapseContent>
            </Collapse>
        </div>
    }
}

#[component]
pub fn AccordionExamples(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <Collapse>
                <CollapseControl slot><input type="radio" name="accordion-example" /></CollapseControl>
                <CollapseTitle slot>Accordion Example</CollapseTitle>
                <CollapseContent slot>This is an example input collapse</CollapseContent>
            </Collapse>
            <Collapse>
                <CollapseControl slot><input type="radio" name="accordion-example" /></CollapseControl>
                <CollapseTitle slot>Accordion Example</CollapseTitle>
                <CollapseContent slot>This is an example input collapse</CollapseContent>
            </Collapse>
            <Collapse>
                <CollapseControl slot><input type="radio" name="accordion-example" /></CollapseControl>
                <CollapseTitle slot>Accordion Example</CollapseTitle>
                <CollapseContent slot>This is an example input collapse</CollapseContent>
            </Collapse>
        </div>
    }
}

#[component]
pub fn AlertExamples(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <Alert>Default Alert</Alert>
            <Alert colour=AlertColour::Colour(Colour::Info)>Info Alert</Alert>
            <Alert colour=AlertColour::Colour(Colour::Success)>Success Alert</Alert>
            <Alert colour=AlertColour::Colour(Colour::Warning)>Warning Alert</Alert>
            <Alert colour=AlertColour::Colour(Colour::Error)>Error Alert</Alert>
        </div>
    }
}

#[component]
pub fn BadgeExamples(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <Badge>Default</Badge>
            <Badge colour=BadgeColour::Colour(Colour::Neutral)>Default</Badge>
            <Badge colour=BadgeColour::Colour(Colour::Primary)>Primary</Badge>
            <Badge colour=BadgeColour::Colour(Colour::Secondary)>Secondary</Badge>
            <Badge colour=BadgeColour::Colour(Colour::Accent)>Accent</Badge>
            <Badge colour=BadgeColour::Ghost()>Ghost</Badge>
            <Badge colour=BadgeColour::Colour(Colour::Info)>Info</Badge>
            <Badge colour=BadgeColour::Colour(Colour::Success)>Success</Badge>
            <Badge colour=BadgeColour::Colour(Colour::Warning)>Warning</Badge>
            <Badge colour=BadgeColour::Colour(Colour::Error)>Error</Badge>
            <Badge outline=true>Default</Badge>
            <Badge outline=true colour=BadgeColour::Colour(Colour::Neutral)>Default</Badge>
            <Badge outline=true colour=BadgeColour::Colour(Colour::Primary)>Primary</Badge>
            <Badge outline=true colour=BadgeColour::Colour(Colour::Secondary)>Secondary</Badge>
            <Badge outline=true colour=BadgeColour::Colour(Colour::Accent)>Accent</Badge>
            <Badge outline=true colour=BadgeColour::Ghost()>Ghost</Badge>
            <Badge outline=true colour=BadgeColour::Colour(Colour::Info)>Info</Badge>
            <Badge outline=true colour=BadgeColour::Colour(Colour::Success)>Success</Badge>
            <Badge outline=true colour=BadgeColour::Colour(Colour::Warning)>Warning</Badge>
            <Badge outline=true colour=BadgeColour::Colour(Colour::Error)>Error</Badge>

            <Badge size=ResponsiveVec(vec![Responsive::Default(BadgeSize::Size(Size::ExtraSmall))])>ExtraSmall</Badge>
            <Badge size=ResponsiveVec(vec![Responsive::Default(BadgeSize::Size(Size::Small))])>Small</Badge>
            <Badge size=ResponsiveVec(vec![Responsive::Default(BadgeSize::Size(Size::Medium))])>Medium/Default</Badge>
            <Badge size=ResponsiveVec(vec![Responsive::Default(BadgeSize::Size(Size::Large))])>Large</Badge>
            <Badge size=ResponsiveVec(vec![Responsive::Small(BadgeSize::Size(Size::Large)), Responsive::Medium(BadgeSize::Size(Size::Medium)), Responsive::Large(BadgeSize::Size(Size::ExtraSmall))])>Responsive</Badge>
        </div>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <ButtonExamples/>
        <DropdownExamples/>
        <ModalExamples/>
        <SwapExamples/>
        <CollapseExamples/>
        <AccordionExamples/>
        <AlertExamples/>
        <BadgeExamples/>
    }
}
