use dioxus::prelude::*;
use std::collections::HashMap;

use crate::{
    components::{
        lib::Color, AboutSection, GithubContributionChart, GithubLanguageBreakdown, GithubRepoCard,
        Tooltip, JournalList, Header
    },
    config::{instance as config, Config},
};

pub fn page(cx: Scope) -> Element {
    let Config { programming, .. } = &config;
    let is_lang_tooltip_open = use_state(&cx, || false);
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

    let contributions_title = {
        let value_class = if total_contributions.get().is_some() {
            "opacity-100"
        } else {
            "opacity-0"
        };

        let skeleton_class = if total_contributions.get().is_some() {
            "opacity-0"
        } else {
            "opacity-100"
        };

        let contributions = if let Some(contributions) = total_contributions.get() {
            contributions
        } else {
            &370
        };

        cx.render(rsx! {
            div {
            h1 {
                class: "text-lg text-tint mb-4 flex items-center relative",
                span {
                    class: "w-7 h-5 skeleton mr-1 rounded {skeleton_class} absolute transition duration-500"
                }
                span {
                    class: "text-dim mr-1 {value_class} transition duration-500",
                    "{contributions}"
                }
                " contributions in the last year"
            }
            }
        })
    };

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
            AboutSection { title: "about me", subtitle: "", text: programming.bio, span_class: "text-moss-dim" }
            div {
                class: "mt-4 relative",
                GithubLanguageBreakdown { github_username: programming.github, colors: colors  }
                div {
                    class: "hidden lg:flex absolute left-0 -ml-6 top-0 h-4 select-none w-4 text-xs items-center justify-center rounded-full border-slate-dim border text-slate-dim cursor-pointer transition opacity-75 hover:opacity-100 duration-300",
                    onmouseover: move |_| { is_lang_tooltip_open.set(true)},
                    onmouseout: move |_| { is_lang_tooltip_open.set(false)},
                    span { "?" }
                    Tooltip {
                        is_open: *is_lang_tooltip_open.get(),
                        rsx!{ p { 
                            class: "text-tint",
                            "this data is fetched by your browser (xhr)."
                        } 
                        p {
                            class: "text-tint mt-2",
                            "component only reads data from my public repositories (no forks n stuffs)."
                        }
                        p {
                            class: "text-slate-dim mt-2",
                            "if this shit doesn't load then my token expired or got rate limited by github, send me an e-mail or open up an issue letting me know and ill give u a present c:"
                        } }
                    }
                }
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
            }
            div {
                class: "w-full mt-2 lg:mt-0 lg:ml-8",
                JournalList {
                    manifest_path: "/static/entries.json",
                }   
            }
        }
    ))
}
