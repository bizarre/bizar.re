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

static GITHUB_TOKEN: &'static str = dotenv!("WEBSITE_GITHUB_TOKEN");

#[derive(PartialEq, Props)]
pub(crate) struct Props<'a> {
    github_username: &'a str,
    colors: &'a UseState<Option<HashMap<String, Color>>>
}

#[derive(PartialEq, Props)]
struct BarProps {
    name: String,
    width: f64,
    color: Option<String>,
    skeleton: Option<bool>
}

#[allow(non_snake_case)]
fn Bar(cx: Scope<BarProps>) -> Element {
    let hovered = use_state(&cx, || false);
    let is_skeleton = cx.props.skeleton.unwrap_or(false);
    let mut width_str = format!("{:3.1}%", cx.props.width);
    let og_width = width_str.clone();
    let color = cx.props.color.clone().unwrap_or("default".to_string());
    let mut more_classes = "cursor-pointer hover:opacity-50 ".to_owned();

    if is_skeleton { more_classes = "skeleton".to_owned(); };

    if *hovered.get() && !is_skeleton {
        width_str = if cx.props.width > 75.0 { width_str } else { "75%".to_string() };
    } 

    let hover_text = {
        let opacity = if *hovered.get() && !is_skeleton { "opacity-100" } else { "opacity-0" };
        rsx!{ label { class:"{opacity} transition duration-200 delay-150 text-xs pointer-events-none", "{cx.props.name} - {og_width}" } }
    };

    cx.render(rsx! { 
        div {
            onmouseover: move |_| { hovered.set(true)},
            onmouseout: move |_| { hovered.set(false)},
            width: "{width_str}",
            class: "h-full transition-all duration-500 opacity-100 transform-gpu {more_classes} flex justify-center",
            background_color: "{color}",
            hover_text
        }
    })
}

#[derive(PartialEq, Props)]
struct BlipProps {
    name: Option<String>,
    percentage: Option<String>,
    color: Option<String>,
}

#[allow(non_snake_case)]
fn Blip(cx: Scope<BlipProps>) -> Element {
    let name = {
        let name = &cx.props.name.clone().unwrap_or_else(|| "".to_string());
        let percentage = &cx
            .props
            .percentage
            .clone()
            .unwrap_or_else(|| "".to_string());

        let skeleton_opacity = if cx.props.name.is_some() {
            "opacity-0"
        } else {
            "opacity-100"
        };

        let text_opacity = if cx.props.name.is_some() {
            "opacity-100"
        } else {
            "opacity-0"
        };

        cx.render(rsx! { div { class: "relative",
            div { class: "absolute {text_opacity} transition-all duration-100", span { "{name} " } span { class: "text-dark block", "{percentage}" } }
            div {
                class: "skeleton h-3 w-5/6 rounded transition-all duration-150 {skeleton_opacity}"
            }
            div {
                class: "skeleton h-3 w-1/3 mt-1.5 rounded transition-all duration-150 {skeleton_opacity}"
            }
        } })
    };

    let dot = {
        let color = &cx.props.color.clone().unwrap_or("white".to_owned());
        let opacity = if cx.props.color.is_some() {
            "opacity-100"
        } else {
            "opacity-0"
        };

        cx.render(rsx! {
            div {
                class: "relative",
                div {
                    class: "mt-1 w-3 h-3 rounded-full mr-2 transition-all duration-1000 {opacity} absolute",
                    background_color: "{color}",
                }
                div {
                    class: "mt-1 w-3 h-3 rounded-full mr-2 skeleton"
                }
            }
        })
    };

    cx.render(rsx! {
        div {
            class: "w-1/5 mb-2 flex select-none transition-all",
            dot
            label {
                class: "w-8/12 text-xs leading-tight",
                name
            }
        }
    })
}

pub(crate) fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let languages = use_state(&cx, || None as Option<(HashMap<String, i64>, i64)>);
    let username = cx.props.github_username.to_owned();
    let colors = cx.props.colors;

    use_future(&cx, (languages,), |(languages,)| async move {
        if languages.is_none() {
            let repo_query = format!(
            "user(login:\"{}\") {{ repos: repositories(last: 100, isFork: false, privacy: PUBLIC, ownerAffiliations: OWNER, isLocked: false) {{ nodes {{ name, isArchived }} }} }}",
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

    let contents = {
        let languages = languages.get();
        let colors = colors.get();
        if let (Some(colors), Some(languages)) = (colors, languages) {
            let (languages, total_bytes) = languages;
            let mut percentages: HashMap<String, f64> = HashMap::new();

            for (language, bytes) in languages {
                let total_bytes = total_bytes.to_owned() as f64;
                let bytes = bytes.to_owned() as f64;
                percentages.insert(language.to_owned(), 100.0 / total_bytes * bytes);
            }

            let sorted = percentages
                .iter()
                // multiply so that close numbers when rounded don't conflict
                .map(|(k, v)| ((v.to_owned() * 100000.0) as i64, k.to_owned()))
                .collect::<BTreeMap<i64, String>>();

            let cloned = sorted.clone();
            let cloned_percentages = percentages.clone();

            (
                rsx! {sorted.iter().rev().enumerate().map(|(i, (_, name))| {
                    let p = percentages.get(name).unwrap_or(&0.0);
                    let x = format!("{:3.1}%", &p);
                    let color = colors.get(name);

                    if let Some(color) = color {
                        let color = color.color.clone();
                        let color = color.unwrap_or("white".to_string());

                        return rsx!(Bar {
                            name: name.to_owned(),
                            key: "{i}-bar",
                            width: *p,
                            color: color
                        });
                    } else {
                        return rsx!(div {
                            key: "{i}-bar",
                            class: "h-full transition-all duration-1000",
                            width: "{x}",
                        });
                    }
                })},
                rsx! { cloned.iter().rev().enumerate().map(|(i, (_, name))| {
                    let p = cloned_percentages.get(name).unwrap_or(&0.0);
                    let percentage = format!("{:3.1}%", p);
                    let color = colors.get(name);
                    if p > &&0.05 {
                        if let Some(color) = color {
                            let color = color.color.clone();
                            rsx!(Blip {
                                key: "{i}-blip",
                                name: name.to_owned(),
                                percentage: percentage,
                                color: color.unwrap_or("white".to_string())
                            })
                        } else {
                            rsx!(Blip {
                                key: "{i}-blip",
                                name: name.to_owned(),
                                color: "white".to_string()
                            })
                        }
                    } else {
                        rsx!(div { key: "{i}-blip" })
                    }
                })},
            )
        } else {
            (
                rsx! { (0..10).map(|i| rsx! { Bar { name: "".to_owned(), key: "{i}-bar", width: 100.0} }) },
                rsx! { (0..10).map(|i| rsx!{ Blip { key: "{i}-blip" }}) },
            )
        }
    };

    let (bar_contents, footer_contents) = contents;

    return {
        cx.render(rsx! {
            div {
                div {
                    class: "w-full h-4 skeleton rounded-full mb-4 flex overflow-hidden opacity-95",
                    bar_contents
                }
                footer {
                    class: "flex justify-start flex-wrap text-dim",
                    footer_contents
                }
            }
        })
    };
}
