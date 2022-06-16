use dioxus::prelude::*;

use crate::{
    components::{AboutSection, GithubLanguageBreakdown},
    config::{instance as config, Config},
};

pub fn page(cx: Scope) -> Element {
    let Config { programming, .. } = &config;

    cx.render(rsx!(
        div {
            class: "w-500",
          AboutSection { subject: "as an engineer", text: programming.bio, span_class: "text-moss-dim" }
          div {
            class: "mt-4",
            h1 {
                class: "text-lg text-dim mb-2",
                "Most used languages"
            }
            GithubLanguageBreakdown { github_username: programming.github  }
          }
        }
    ))
}
