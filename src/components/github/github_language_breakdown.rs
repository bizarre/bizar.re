use crate::components::github::lib::*;
use dioxus::{prelude::*, router::*};
use dotenv_codegen::*;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{
    any::Any,
    collections::{BTreeMap, HashMap},
};

static GITHUB_TOKEN: &'static str = dotenv!("GITHUB_TOKEN");

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    github_username: &'a str,
}

#[derive(PartialEq, Props)]
struct BlipProps {
    name: Option<String>,
    color: Option<String>,
}

fn Blip(cx: Scope<BlipProps>) -> Element {
    let name = if let Some(name) = &cx.props.name {
        cx.render(rsx! { "{name}" })
    } else {
        cx.render(rsx! { div {
            class: "skeleton h-4 rounded"
        } })
    };

    let dot = if let Some(color) = &cx.props.color {
        cx.render(rsx! {   div {
            background_color: "{color}",
            class: "w-3 h-3 rounded-full mr-2"
        } })
    } else {
        cx.render(rsx! {   div {
            class: "w-3 h-3 rounded-full skeleton mr-2"
        } })
    };

    cx.render(rsx! {
        div {
            class: "w-1/3 mb-2 flex items-center",
            dot
            label {
                class: "w-10/12 text-sm",
                name
            }
        }
    })
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct Color {
    pub color: Option<String>,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let loaded = use_state(&cx, || false);
    let colors = use_state(&cx, || None);
    let languages = use_state(&cx, || None);
    let username = cx.props.github_username.to_owned();

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

    use_future(&cx, (languages,), |(languages,)| async move {
        if (languages.is_none()) {
            let repo_query = format!(
            "user(login:\"{}\") {{ name: repositories(last: 30) {{ nodes {{ name isFork }} }} }}",
            &username
        )
            .replace("\n", "")
            .replace("\"", "\\\"");

            let body = format!(
                "
            {{ \"query\": \"query {{ {} }} \" }}",
                repo_query
            );

            let client = reqwest::Client::new();
            let resp = client
                .post("https://api.github.com/graphql")
                .bearer_auth(GITHUB_TOKEN)
                .body(body)
                .send()
                .await
                .unwrap()
                .json::<RepoResponse>()
                .await
                .unwrap();

            let languages_map = resp.get_languages(username, GITHUB_TOKEN).await.unwrap();
            languages.set(Some(languages_map));
        }
    });

    use_future(
        &cx,
        (languages, colors, loaded),
        |(languages, colors, loaded)| async move {
            if languages.is_some() && colors.is_some() && !&loaded {
                loaded.set(true);
            }
        },
    );

    if !**loaded {
        return cx.render(rsx! {
            div {
                div {
                    class: "w-full h-4 bg-dim rounded-full mb-4 skeleton"

                }
                footer {
                    class: "flex justify-between flex-wrap text-dim",
                    Blip { }
                    Blip {  }
                    Blip {  }
                    Blip {  }
                }
            }
        });
    } else {
        let languages = languages.current();
        let (languages, total_bytes) = languages.as_ref().as_ref().unwrap().clone();
        let colors = colors.current().as_ref().as_ref().unwrap().clone();
        let mut percentages: HashMap<String, f64> = HashMap::new();

        for (language, bytes) in languages {
            let total_bytes: f64 = total_bytes as f64;
            let bytes: f64 = bytes as f64;
            percentages.insert(language, 100.0 / total_bytes * bytes);
        }

        let sorted: BTreeMap<i64, String> = percentages
            .iter()
            .map(|(k, v)| (v.to_owned() as i64, k.to_owned()))
            .collect();

        return {
            cx.render(rsx! {
                div {
                    div {
                        class: "w-full h-4 bg-dim rounded-full mb-4 flex overflow-hidden",
                        sorted.iter().rev().map(|(_, name)| {
                            let p = percentages.get(name).unwrap();
                            let x = format!("{:3.1}%", p.ceil());
                            let cloned = colors.clone();
                            let color = cloned.get(name);

                            if let Some(color) = color {
                                let color = color.color.clone();
                                let color = color.unwrap_or("white".to_string());

                                   return rsx!(
                                        div {
                                            class: "h-full",
                                            width: "{x}",
                                            background_color: "{color}"
                                        }
                                    );
                            } else {
                                return rsx!(div{});
                            }

                         
                        })
                    }
                    footer {
                        class: "flex justify-between flex-wrap text-dim",
                         sorted.iter().rev().map(|(_, name)| {
                            let p = percentages.get(name).unwrap();
                            let x = format!("{} {:3.1}%", name, p);
                            let cloned = colors.clone();
                            let color = cloned.get(name);
                            if let Some(color) = color {
                                let color = color.color.clone();
                                rsx!(
                                    Blip { key: "{name}", name: x, color: color.unwrap_or("white".to_string()) }
                                )
                            } else {
                                rsx!(
                                    Blip { name: x }
                                )
                            }
                        })
                    }
                }
            })
        };
    }
}
