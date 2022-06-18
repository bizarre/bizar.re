use dioxus::prelude::*;
use std::collections::HashMap;

use crate::{
    components::{
        lib::Color, AboutSection, GithubContributionChart, GithubLanguageBreakdown, GithubRepoCard,
    },
    config::{instance as config, Config},
};

pub fn page(cx: Scope) -> Element {
    let Config { programming, .. } = &config;
    let total_contributions = use_state(&cx, || None as Option<i64>);
    let colors = use_state(&cx, || None as Option<HashMap<String, Color>>);

    use_future(&cx, (colors,), |(colors,)| async move {
        if colors.is_none() {
            let colors_map = reqwest::get(
                "https://raw.githubusercontent.com/ozh/github-colors/master/colors.json",
            )
            .await
            .unwrap()
            .json::<HashMap<String, Color>>()
            .await
            .unwrap();

            colors.set(Some(colors_map));
        }
    });

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
            class: "w-full lg:w-500",
          AboutSection { title: "About me", subtitle: "", text: programming.bio, span_class: "text-moss-dim" }
          div {
            class: "mt-4",
            GithubLanguageBreakdown { github_username: programming.github, colors: colors  }
          }
          div {
            class: "mt-4 outline-moss outline-2 w-full",
            contributions_title
            GithubContributionChart { github_username: programming.github, contributions: total_contributions }
          }
          div {
            class: "mt-6",
            programming.github_repos.iter().map(|repo| rsx! {
                GithubRepoCard {
                    key: "{repo}",
                    github_repo: repo,
                    colors: colors
                }
            })
          }
        }
    ))
}
