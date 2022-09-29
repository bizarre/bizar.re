use dioxus::prelude::*;

use crate::{
    components::{AboutSection, Header},
    config::{instance as config, Config},
};

pub fn page(cx: Scope) -> Element {
    let Config { photography, .. } = &config;

    cx.render(rsx!(
        div {
            class: "w-full flex flex-col lg:flex-row justify-between",
            div {
                div {
                    class: "w-full lg:w-500",
                    Header { name: config.name, pseudonym: config.pseudonym, headline: config.headline, social: &config.social }
                }
                div {
                    class: "w-full lg:w-500",
                    AboutSection { title: "about me", subtitle: "", text: photography.bio, span_class: "text-moss-dim" }
                    div {
                        class: "mt-3",
                        h1 {
                            class: "text-xl text-tint",
                            ""
                        }
                    }
                    div {
                        class: "p-3 bg-dim w-full text-tint rounded",
                        span {
                            "Sony A7 III"
                        }
                    }
                }
            }
        }
    ))
}
