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
            class: "w-full p-12 flex justify-center",
            div {
                class: "w-full flex flex-col items-center",
                div {
                    class: "w-500",
                     Header { name: config.name, pseudonym: config.pseudonym, headline: config.headline }
                }
               
                &cx.props.children
            }
        }
    })
}
