use dioxus::prelude::*;

use crate::{
    components::Header,
    config::{instance as config, Config},
};

pub fn page(cx: Scope) -> Element {
    let Config {
        name,
        pseudonym,
        headline,
        ..
    } = config;

    cx.render(rsx!(
        div {
            Header { name: name, pseudonym: pseudonym, headline: headline }
        }
    ))
}
