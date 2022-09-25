use crate::components::github::lib::*;
use dioxus::{prelude::*, router::*};
use dotenv_codegen::*;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{
    any::Any,
    collections::{BTreeMap, HashMap},
    panic,
};

static GITHUB_TOKEN_ENCODED: &'static str = dotenv!("WEBSITE_GITHUB_TOKEN");

#[derive(Deserialize, PartialEq)]
struct LanguageEdgeNode {
    pub name: String,
}

#[derive(Deserialize, PartialEq)]
struct LanguageEdge {
    pub node: LanguageEdgeNode,
}

#[derive(Deserialize, PartialEq)]
struct LanguageEdges {
    edges: Vec<LanguageEdge>,
}

#[derive(Deserialize, PartialEq)]
struct RepoInfoRepository {
    #[serde(alias = "stargazerCount")]
    pub stars: u32,
    pub description: Option<String>,
    #[serde(alias = "forkCount")]
    pub fork_count: u32,
    pub languages: LanguageEdges,
}

#[derive(Deserialize, PartialEq)]
struct RepoInfoResponse {
    repository: RepoInfoRepository,
}

#[derive(Deserialize, PartialEq)]
struct RepoInfoResponseContainer {
    data: RepoInfoResponse,
}

#[inline_props]
pub(crate) fn component<'a>(
    cx: Scope,
    github_repo: &'a str,
    colors: &'a UseState<Option<HashMap<String, Color>>>,
) -> Element {
    let parts: Vec<String> = github_repo.split("/").map(|d| d.to_owned()).collect();
    let username = parts.clone().get(0).unwrap().clone();
    let username_cloned = username.clone();
    let repo_name = parts.clone().get(1).unwrap().clone();
    let repo_name_cloned = repo_name.clone();
    let repo = use_state(&cx, || None as Option<RepoInfoRepository>);

    use_future(&cx, (repo,), |(repo,)| async move {
        let username = username.clone();

        if repo.is_some() {
            return;
        }

        let repo_query = format!(
            "repository(owner:\"{}\", name:\"{}\") {{
                stargazerCount,
                description,
                forkCount,
                languages(first:50) {{
                edges {{
                    node {{
                    name
                    }}
                }}
            }}
        }}",
            &username, &repo_name
        )
        .replace("\n", "")
        .replace("\"", "\\\"");

        let body = format!(
            "
            {{ \"query\": \"query {{ {} }} \" }}",
            repo_query
        );

        let decoded = base64::decode(&GITHUB_TOKEN_ENCODED).unwrap();
        let decoded = std::str::from_utf8(&decoded).unwrap();

        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.github.com/graphql")
            .bearer_auth(decoded)
            .body(body)
            .send()
            .await
            .unwrap()
            .json::<RepoInfoResponseContainer>()
            .await
            .unwrap();

        repo.set(Some(resp.data.repository));
    });

    let skeleton_class = if repo.get().is_some() {
        "opacity-0"
    } else {
        "opacity-100"
    };

    let value_class = if repo.get().is_some() {
        "opacity-100"
    } else {
        "opacity-0 text-transparent"
    };

    let name_content = {
        rsx! {
            div {
                class: "flex items-center mb-2",
                object {
                    class: "absolute mr-2",
                    data:"/static/svg/bookmark.svg",
                    max_height: "18px",
                    max_width: "18px"
                }
                div {
                    class: "relative ml-6",
                    div {
                        class: "w-full h-1/2 mt-1.5 skeleton rounded-l transition duration-500 {skeleton_class} absolute"
                    }
                    h1 {
                        class: "transition {value_class} text-lg text-slate delay-300 duration-500",
                        "{username_cloned}/ "
                    }
                }
                div {
                    class: "relative",
                     div {
                        class: "w-full h-1/2 mt-1.5 -ml-2 skeleton-light rounded-r duration-500 transition {skeleton_class} absolute"
                    }
                    h1 {
                        class: "transition {value_class} text-lg text-dim delay-300 duration-500",
                        "{repo_name_cloned}"
                    }
                }
            }
        }
    };

    let description_content = {
        let description = if let Some(repo) = repo.get() {
            let desc = repo.description.clone().unwrap_or("".to_owned()).to_owned();
            rsx! { "{desc}" }
        } else {
            // lol this is so wack LMAO
            rsx! { br {} br{} }
        };

        rsx! {
            div {
                class: "flex relative",
                div {
                    class: "ml-6 w-full relative",
                    (0..2).map(|i| {
                    let width = 100.0 - (i as f64 * 20.0);
                    let width_str = format!("{:3.1}%", width);
                    let mut margin_top = format!("{}rem", i as f64 * 0.75);

                    if i == 0 {
                        margin_top = "0.1rem".to_string();
                    }

                    rsx!{
                        div {
                            key: "{i}",
                            class: "h-2 skeleton rounded transition duration-500 {skeleton_class} absolute",
                            width: "{width_str}",
                            margin_top: "{margin_top}"
                        }
                    }})


                    p {
                        class: "text-dim text-xs {value_class} transition",
                        style: "line-height: 1rem;",
                        description
                    }
                }
            }
        }
    };

    let footer_content = {
        let language = if let Some(repo) = repo.get() {
            repo.languages
                .edges
                .first()
                .map_or("Failed to load", |x| &x.node.name)
        } else {
            "Loading"
        };

        let stars = if let Some(repo) = repo.get() {
            format!("{}", repo.stars)
        } else {
            "0".to_string()
        };

        let forks = if let Some(repo) = repo.get() {
            format!("{}", repo.fork_count)
        } else {
            "0".to_string()
        };

        let color = if let Some(colors) = colors.get() {
            colors
                .get(language)
                .map_or("white", |c| c.color.as_ref().unwrap())
        } else {
            ""
        };

        rsx! {
            footer {
                class: "ml-6 flex mt-2 items-center",
                div {
                    style: "line-height: 0;",
                    class: "flex",
                    div {
                        class: "relative",
                        div {
                            class: "w-2 h-2 {skeleton_class} skeleton rounded-full absolute transition"
                        }
                        div {
                            background_color: "{color}",
                            class: "w-2 h-2 {value_class} rounded-full transition-all duration-500 mt-1"
                        }
                    }
                    div {
                        class: "relative",
                        div {
                            class: "w-12 h-2 {skeleton_class} skeleton absolute transition rounded ml-1"
                        }
                        label {
                            class: "{value_class} transition text-tint text-xs duration-500 ml-1",
                            "{language}"
                        }
                    }
                }
                div {
                    class: "flex ml-2",
                    style: "line-height: 0",
                    div {
                        class: "relative",
                        div {
                            class: "w-2 h-2 {skeleton_class} skeleton rounded-full absolute transition ml-1"
                        }
                        object {
                            class: "{value_class} transition mt-0.5",
                            data:"/static/svg/star.svg",
                            max_height: "0.75rem",
                            max_width: "0.75rem"
                        }
                    }
                    div {
                        class: "relative",
                        div {
                            class: "w-5 h-2 {skeleton_class} skeleton absolute transition rounded ml-1"
                        }
                        label {
                            class: "{value_class} transition text-tint text-xs duration-500",
                            "{stars}"
                        }
                    }
                }
                div {
                    class: "flex ml-2",
                    style: "line-height: 0;",
                    div {
                        class: "relative",
                        div {
                            class: "w-2 h-2 {skeleton_class} skeleton rounded-full absolute transition ml-4"
                        }
                        object {
                            class: "{value_class} transition mt-0.5",
                            data:"/static/svg/branch.svg",
                            max_height: "0.75rem",
                            max_width: "0.75rem"
                        }
                    }
                    div {
                        class: "relative",
                        div {
                            class: "w-5 h-2 {skeleton_class} skeleton absolute transition rounded ml-4"
                        }
                        label {
                            class: "{value_class} transition text-tint text-xs duration-500",
                            "{forks}"
                        }
                    }
                }
            }
        }
    };

    let mut container_class =
        "transition transform-gpu hover:scale-102 hover:opacity-75 cursor-pointer";
    if repo.is_none() {
        container_class = "";
    }

    let href = format!("https://github.com/{}", cx.props.github_repo);

    cx.render(rsx! {
        a {
            class: "block w-full rounded-lg {container_class} mt-2 first:mt-0 pr-10 pb-3 last:pb-0",
            href: "{href}",
            target: "_blank",
            name_content
            description_content
            footer_content
        }
    })
}
