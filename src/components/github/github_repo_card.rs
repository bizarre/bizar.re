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
                        class: "w-full h-2/3 mt-1 skeleton-moss rounded-l transition duration-500 {skeleton_class} absolute"
                    }
                    h1 {
                        class: "transition {value_class} text-lg text-slate delay-300 duration-500",
                        "{username_cloned}/ "
                    }
                }
                div {
                    class: "relative",
                     div {
                        class: "w-full h-2/3  mt-1 -ml-2 skeleton-moss-light rounded-r duration-500 transition {skeleton_class} absolute"
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
            rsx! { br {} br{} br{} }
        };

        rsx! {
            div {
                class: "flex relative",
                div {
                    class: "ml-6 w-full relative",
                    (0..2).map(|i| {
                    let width = 100.0 - (i as f64 * 20.0);
                    let width_str = format!("{:3.1}%", width);
                    let mut margin_top = format!("{}rem", i as f64);

                    if i == 0 {
                        margin_top = "0.1rem".to_string();
                    }

                    rsx!{
                        div {
                            key: "{i}",
                            class: "h-3 skeleton-moss rounded transition duration-500 {skeleton_class} absolute",
                            width: "{width_str}",
                            margin_top: "{margin_top}"
                        }
                    }})


                    p {
                        class: "text-dim text-xs {value_class} transition",
                        style: "line-height: 0.75rem;",
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
                .map_or("Loading", |x| &x.node.name)
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
                    class: "flex items-center",
                    style: "line-height: 0.75rem;",
                    div {
                        class: "relative mr-1.5",
                        div {
                            class: "w-3 h-3 {skeleton_class} skeleton-moss rounded-full absolute transition"
                        }
                        div {
                            background_color: "{color}",
                            class: "w-3 h-3 {value_class} rounded-full transition-all duration-500"
                        }
                    }
                    div {
                        class: "relative",
                        div {
                            class: "w-10 h-4 {skeleton_class} skeleton-moss absolute transition rounded"
                        }
                        label {
                            class: "{value_class} transition text-dark text-xs duration-500",
                            "{language}"
                        }
                    }
                }
                div {
                    class: "flex items-center ml-2",
                    style: "line-height: 0.75rem;",
                    div {
                        class: "relative",
                        div {
                            class: "w-3 h-3 {skeleton_class} skeleton-moss rounded-full absolute transition"
                        }
                        object {
                            class: "{value_class} transition",
                            data:"/static/svg/star.svg",
                            max_height: "16px",
                            max_width: "16px"
                        }
                    }
                    div {
                        class: "relative",
                        div {
                            class: "w-5 h-4 {skeleton_class} skeleton-moss absolute transition rounded"
                        }
                        label {
                            class: "{value_class} transition text-dark text-xs duration-500",
                            "{stars}"
                        }
                    }
                }
                 div {
                    class: "flex items-center ml-2",
                    style: "line-height: 1rem;",
                    div {
                        class: "relative",
                        div {
                            class: "w-3 h-3 ml-4 {skeleton_class} skeleton-moss rounded-full absolute transition"
                        }
                        object {
                            class: "{value_class} transition",
                            data:"/static/svg/branch.svg",
                            max_height: "16px",
                            max_width: "16px"
                        }
                    }
                    div {
                        class: "relative",
                        div {
                            class: "w-5 h-4 ml-4 {skeleton_class} skeleton-moss absolute transition rounded"
                        }
                        label {
                            class: "{value_class} transition text-dark text-xs duration-500",
                            "{forks}"
                        }
                    }
                }
            }
        }
    };

    let mut container_class = "transition transform-gpu hover:scale-105 cursor-pointer";
    if repo.is_none() {
        container_class = "";
    }

    let href = format!("https://github.com/{}", cx.props.github_repo);

    cx.render(rsx! {
        a {
            class: "block bg-moss-dark w-full p-4 rounded-lg {container_class} mt-4 pr-10 pb-6",
            href: "{href}",
            target: "_blank",
            name_content
            description_content
            footer_content
        }
    })
}
