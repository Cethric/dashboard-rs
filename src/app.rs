use leptos::html::Dialog;
use leptos::*;
use leptos_icons::{BsIcon, Icon};
use log::info;
use wasm_bindgen::prelude::*;

use components::alert::*;
use components::badge::*;
use components::button::*;
use components::collapse::*;
use components::dropdown::*;
use components::modal::*;
use components::swap::*;
use components::util::*;

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
            <Button variant=ButtonVariant::Neutral>Neutral</Button>
            <Button variant=ButtonVariant::Primary>Primary</Button>
            <Button variant=ButtonVariant::Secondary>Secondary</Button>
            <Button variant=ButtonVariant::Accent>Accent</Button>
            <Button variant=ButtonVariant::Ghost>Ghost</Button>
            <Button variant=ButtonVariant::Link>Link</Button>
        </div>
        <div>
            <Button active=true>Default Active</Button>
            <Button active=true variant=ButtonVariant::Neutral>Neutral Active</Button>
            <Button active=true variant=ButtonVariant::Primary>Primary Active</Button>
            <Button active=true variant=ButtonVariant::Secondary>Secondary Active</Button>
            <Button active=true variant=ButtonVariant::Accent>Accent Active</Button>
            <Button active=true variant=ButtonVariant::Ghost>Ghost Active</Button>
            <Button active=true variant=ButtonVariant::Link>Link Active</Button>
        </div>
        <div>
            <Button variant=ButtonVariant::Info>Info</Button>
            <Button variant=ButtonVariant::Success>Success</Button>
            <Button variant=ButtonVariant::Warning>Warning</Button>
            <Button variant=ButtonVariant::Error>Error</Button>
        </div>
        <div>
            <Button outline=true>Default</Button>
            <Button outline=true variant=ButtonVariant::Primary>Primary</Button>
            <Button outline=true variant=ButtonVariant::Secondary>Secondary</Button>
            <Button outline=true variant=ButtonVariant::Accent>Accent</Button>
        </div>
        <div>
            <Button outline=true variant=ButtonVariant::Info>Info</Button>
            <Button outline=true variant=ButtonVariant::Success>Success</Button>
            <Button outline=true variant=ButtonVariant::Warning>Warning</Button>
            <Button outline=true variant=ButtonVariant::Error>Error</Button>
        </div>
        <div>
            <Button size=ButtonSize::Large>Large</Button>
            <Button >Default</Button>
            <Button size=ButtonSize::Small>Small</Button>
            <Button size=ButtonSize::Tiny>Tiny</Button>
        </div>
        <div>
            <Button wide=true>Default</Button>
        </div>
        <div>
            <Button disabled=true>Default</Button>
        </div>
        <div>
            <Button shape=ButtonShape::Square>X</Button>
            <Button shape=ButtonShape::Square outline=true>X</Button>
        </div>
        <div>
            <Button shape=ButtonShape::Circle>X</Button>
            <Button shape=ButtonShape::Circle outline=true>X</Button>
        </div>
        <div>
            <Button responsive_shape=ResponsiveVec(vec![Responsive::Large(ButtonShape::Circle), Responsive::Small(ButtonShape::Square)])>X</Button>
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
            <Alert variant=AlertVariant::Info>Info Alert</Alert>
            <Alert variant=AlertVariant::Success>Success Alert</Alert>
            <Alert variant=AlertVariant::Warning>Warning Alert</Alert>
            <Alert variant=AlertVariant::Error>Error Alert</Alert>
        </div>
    }
}

#[component]
pub fn BadgeExamples(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <Badge>Default</Badge>
            <Badge variant=BadgeVariant::Neutral>Default</Badge>
            <Badge variant=BadgeVariant::Primary>Primary</Badge>
            <Badge variant=BadgeVariant::Secondary>Secondary</Badge>
            <Badge variant=BadgeVariant::Accent>Accent</Badge>
            <Badge variant=BadgeVariant::Ghost>Ghost</Badge>
            <Badge variant=BadgeVariant::Info>Info</Badge>
            <Badge variant=BadgeVariant::Success>Success</Badge>
            <Badge variant=BadgeVariant::Warning>Warning</Badge>
            <Badge variant=BadgeVariant::Error>Error</Badge>
            <Badge outline=true>Default</Badge>
            <Badge outline=true variant=BadgeVariant::Neutral>Default</Badge>
            <Badge outline=true variant=BadgeVariant::Primary>Primary</Badge>
            <Badge outline=true variant=BadgeVariant::Secondary>Secondary</Badge>
            <Badge outline=true variant=BadgeVariant::Accent>Accent</Badge>
            <Badge outline=true variant=BadgeVariant::Ghost>Ghost</Badge>
            <Badge outline=true variant=BadgeVariant::Info>Info</Badge>
            <Badge outline=true variant=BadgeVariant::Success>Success</Badge>
            <Badge outline=true variant=BadgeVariant::Warning>Warning</Badge>
            <Badge outline=true variant=BadgeVariant::Error>Error</Badge>

            <Badge size=BadgeSize::Tiny>Tiny</Badge>
            <Badge size=BadgeSize::Small>Small</Badge>
            <Badge size=BadgeSize::Medium>Medium/Default</Badge>
            <Badge size=BadgeSize::Large>Large</Badge>
            <Badge responsive=ResponsiveVec(vec![Responsive::Small(BadgeSize::Large), Responsive::Medium(BadgeSize::Medium), Responsive::Large(BadgeSize::Tiny)])>Responsive</Badge>
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
