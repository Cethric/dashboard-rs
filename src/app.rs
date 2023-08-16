use chrono::{Datelike, Local, Timelike};
use leptos::*;
use leptos_icons::{BsIcon, Icon};
use leptos_use::use_interval_fn;
use log::info;
use wasm_bindgen::prelude::*;

use components::actions::button::{Button, ButtonColour, ButtonSize};
use components::colour::Colour;
use components::common::TextBackgroundColour;
use components::data_display::countdown::Countdown;
use components::navigation::bottom_navigation::{
    BottomNavigation, BottomNavigationItem, BottomNavigationSize,
};
use components::navigation::navbar::{Navbar, NavbarCenter, NavbarEnd, NavbarStart};
use components::responsive::{Responsive, ResponsiveVec};
use components::size::Size;
use components::typography::TextColour;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Copy, Clone)]
struct CurrentTime {
    day: (ReadSignal<i32>, WriteSignal<i32>),
    month: (ReadSignal<i32>, WriteSignal<i32>),
    year: (ReadSignal<i32>, WriteSignal<i32>),
    hour: (ReadSignal<i32>, WriteSignal<i32>),
    minute: (ReadSignal<i32>, WriteSignal<i32>),
    second: (ReadSignal<i32>, WriteSignal<i32>),
}

#[component]
pub fn Clock(cx: Scope) -> impl IntoView {
    let current_time: CurrentTime = CurrentTime {
        day: create_signal::<i32>(cx, 0),
        month: create_signal::<i32>(cx, 0),
        year: create_signal::<i32>(cx, 0),
        hour: create_signal::<i32>(cx, 0),
        minute: create_signal::<i32>(cx, 0),
        second: create_signal::<i32>(cx, 0),
    };
    use_interval_fn(
        cx,
        move || {
            let current = Local::now();
            current_time.day.1.set(
                i32::try_from(current.day())
                    .ok()
                    .expect("invalid conversion from u32 to i32"),
            );
            current_time.month.1.set(
                i32::try_from(current.month())
                    .ok()
                    .expect("invalid conversion from u32 to i32"),
            );
            current_time.year.1.set(current.year() - 2000);
            current_time.hour.1.set(
                i32::try_from(current.hour())
                    .ok()
                    .expect("invalid conversion from u32 to i32"),
            );
            current_time.minute.1.set(
                i32::try_from(current.minute())
                    .ok()
                    .expect("invalid conversion from u32 to i32"),
            );
            current_time.second.1.set(
                i32::try_from(current.second())
                    .ok()
                    .expect("invalid conversion from u32 to i32"),
            );
        },
        1000,
    );

    view! {cx
        <div>
            <Countdown value=MaybeSignal::Dynamic(current_time.day.0.derive_signal(cx)) />
            <span>/</span>
            <Countdown value=MaybeSignal::Dynamic(current_time.month.0.derive_signal(cx)) />
            <span>/</span>
            <Countdown value=MaybeSignal::Dynamic(current_time.year.0.derive_signal(cx)) />
            <span> -- </span>
            <Countdown value=MaybeSignal::Dynamic(current_time.hour.0.derive_signal(cx)) />
            <span>:</span>
            <Countdown value=MaybeSignal::Dynamic(current_time.minute.0.derive_signal(cx)) />
            <span>:</span>
            <Countdown value=MaybeSignal::Dynamic(current_time.second.0.derive_signal(cx)) />
        </div>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    info!("Hello, World");

    view! { cx,
        <Navbar colour=TextBackgroundColour::Colour(Colour::Primary)>
            <NavbarStart slot>
                <Button colour=ButtonColour::Colour(Colour::Secondary) size=ResponsiveVec(vec![Responsive::Default(ButtonSize::Size(Size::Small))])>SlideShow</Button>
            </NavbarStart>
            <NavbarCenter slot>
                <Clock/>
            </NavbarCenter>
            <NavbarEnd slot>
                <Button colour=ButtonColour::Colour(Colour::Secondary) size=ResponsiveVec(vec![Responsive::Default(ButtonSize::Size(Size::Small))])>Edit</Button>
            </NavbarEnd>
        </Navbar>
        <BottomNavigation size=ResponsiveVec(vec![Responsive::Default(BottomNavigationSize::Size(Size::Small)), Responsive::Small(BottomNavigationSize::Size(Size::Medium)), Responsive::ExtraExtraLarge(BottomNavigationSize::Size(Size::Large))])>
            <BottomNavigationItem slot colour=TextColour::Colour(Colour::Primary) icon=Icon::Bs(BsIcon::BsGithub) active=MaybeSignal::Static(false) disabled=MaybeSignal::Static(false)/>
            <BottomNavigationItem slot colour=TextColour::Colour(Colour::Primary) icon=Icon::Bs(BsIcon::BsGithub) active=MaybeSignal::Static(false) disabled=MaybeSignal::Static(false)/>
            <BottomNavigationItem slot colour=TextColour::Colour(Colour::Primary) icon=Icon::Bs(BsIcon::BsGithub) active=MaybeSignal::Static(false) disabled=MaybeSignal::Static(false)/>
            <BottomNavigationItem slot colour=TextColour::Colour(Colour::Primary) icon=Icon::Bs(BsIcon::BsGithub) active=MaybeSignal::Static(false) disabled=MaybeSignal::Static(false)/>
            <BottomNavigationItem slot colour=TextColour::Colour(Colour::Primary) icon=Icon::Bs(BsIcon::BsGithub) active=MaybeSignal::Static(false) disabled=MaybeSignal::Static(false)/>
            <BottomNavigationItem slot colour=TextColour::Colour(Colour::Primary) icon=Icon::Bs(BsIcon::BsPower) active=MaybeSignal::Static(false) disabled=MaybeSignal::Static(false)/>
        </BottomNavigation>
    }
}
