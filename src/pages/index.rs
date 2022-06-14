use dioxus::prelude::*;

use crate::components::Header;
use crate::config::instance as config;

pub fn page(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            Header { name: config.name }
        }
    ))
}
