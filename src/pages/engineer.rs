use dioxus::prelude::*;

use crate::{
    components::{AboutSection, GithubContributionChart, GithubLanguageBreakdown},
    config::{instance as config, Config},
};

pub fn page(cx: Scope) -> Element {
    let Config { programming, .. } = &config;
    let total_contributions = use_state(&cx, || None as Option<i64>);

    let contributions_title = match total_contributions.get() {
        Some(contributions) => cx.render(rsx! {
             h1 {
                class: "text-xl text-tint mb-4 flex items-center",
                span {
                    class: "text-dim mr-1",
                    "{contributions} "
                }
                " contributions in the last year"
            }
        }),
        None => cx.render(rsx! {
            h1 {
                class: "text-xl text-tint mb-4 flex items-center",
                span {
                    class: "w-7 h-5 skeleton mr-1 rounded"
                }
                "contributions in the last year"
            }
        }),
    };

    cx.render(rsx!(
        div {
            class: "w-500",
          AboutSection { subject: "as an engineer", text: programming.bio, span_class: "text-moss-dim" }
          div {
            class: "mt-4",
            GithubLanguageBreakdown { github_username: programming.github  }
          }
          div {
            class: "mt-4 transition outline-moss outline-2 w-full",
            contributions_title
            GithubContributionChart { github_username: programming.github, contributions: total_contributions }
          }
        }
    ))
}
