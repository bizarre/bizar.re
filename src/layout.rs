use dioxus::prelude::*;

use crate::{components::Header, config::instance as config};

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn DefaultLayout<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full p-16 flex justify-center",
            div {
                Header { name: config.name, pseudonym: config.pseudonym, headline: config.headline }
                &cx.props.children
            }
        }
    })
}
