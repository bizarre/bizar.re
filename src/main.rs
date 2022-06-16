use dioxus::prelude::*;

mod components;
mod config;
mod layout;
mod pages;
mod router;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    router::route(cx)
}
