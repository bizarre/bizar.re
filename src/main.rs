use dioxus::prelude::*;

mod components;
mod config;
mod layout;
mod pages;
mod router;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    router::route(cx)
}
