use dioxus::{prelude::*, router::*};
use serde::Deserialize;

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    manifest_path: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct JournalEntry {
    title: String,
    subtitle: String,
    date: String,
    preview: Option<String>,
    #[serde(rename = "_stem")]
    pub link: String,
    #[serde(rename = "_path")]
    pub path: String,
}

#[inline_props]
fn SkeletonStub(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full mb-4 rounded",
            div {
                class: "w-1/2 h-3 skeleton rounded mb-2"
            }
            (1..5).rev().map(|i| {
                let width = 20.0 * ((i+1) as f64);
                let width = format!("{}%", width);
                rsx! {
                div {
                    style: "width: {width}",
                    class: "skeleton-light h-2 mb-1 rounded"
                }
            }})
        }
    })
}

#[inline_props]
fn EntryStub(cx: Scope, entry: JournalEntry) -> Element {
    cx.render(rsx! {
        Link {
            to: "{entry.link}",
            div {
                class: "w-full relative mb-4 rounded group cursor-pointer text-sm transform transition hover:scale-102 bg-black",
                div {
                    class: "w-full opacity-0 group-hover:opacity-100 transition transition-500 absolute h-full rounded z-40 flex justify-center items-center backdrop-blur",
                    h1 {
                        class: "lowercase text-xs font-medium text-dim text-black",
                        "🤓📖"
                    }
                }

                h1 {
                    class: "text-slate font-medium select-none flex group-hover:opacity-75 transition flex items-center w-full",
                    "{entry.title}"
                    h2 {
                        class: "ml-1 text-slate-dim",
                        "{entry.subtitle}"
                    }
                    label {
                        class: "text-slate-dim text-xs cursor-pointer group-hover:opacity-75 transition ml-auto",
                        "{entry.date}"
                    }
                }
                p {
                    class: "w-full text-dim text-xs leading-tight overflow-hidden text-ellipsis group-hover:opacity-75 transition",
                    style: "line-clamp: 3; display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 3; text-overflow: ellipsis;",
                    entry.preview.as_ref().map(|x| rsx!{ span { "{x}" }})
                }
            }
        }
    })
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let manifest = use_state(&cx, || None as Option<Vec<JournalEntry>>);
    let base_url = web_sys::window().unwrap().origin();
    let manifest_path = cx.props.manifest_path.to_owned();

    use_future(&cx, (manifest,), |(manifest,)| async move {
        if manifest.is_some() {
            return;
        }

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}{}", base_url, manifest_path))
            .send()
            .await
            .unwrap()
            .json::<Vec<JournalEntry>>()
            .await
            .unwrap();

        manifest.set(Some(resp.clone()));
    });

    cx.render(rsx! {
        section {
            class: "flex flex-col w-full mt-4 lg:mt-0 pb-12 lg:pb-0",
            manifest.is_none().then(|| rsx! { (0..3).map(|_| rsx! {
                SkeletonStub {  }
            })})

            manifest.is_some().then(|| rsx! { manifest.get().as_ref().unwrap().iter().map(|entry| rsx! {
                EntryStub { entry: entry.clone() }
            }) })
        }
    })
}
