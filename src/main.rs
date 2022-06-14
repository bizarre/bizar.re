use dioxus::prelude::*;

mod components;
mod layout;
mod pages;
mod router;
mod config;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    router::route(cx)
}
