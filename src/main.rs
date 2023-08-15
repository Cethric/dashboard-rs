#![feature(stmt_expr_attributes)]

use console_error_panic_hook::set_once;
use console_log::init_with_level;
use leptos::*;
use log::{info, Level};

use app::*;

mod app;

fn main() {
    init_with_level(Level::Debug).expect("failed to set log level");
    set_once();

    info!("Hello, World!");

    mount_to_body(|cx| {
        view! { cx,
            <App />
        }
    })
}
