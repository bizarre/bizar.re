use dioxus::{
    prelude::{dioxus_elements::base, *},
    router::{use_route, Link, Redirect, Route, Router},
};
use serde::Deserialize;

use crate::components::journal::{
    JournalEntry as JournalEntryFragment, JournalEntryBody, JournalEntryHeader,
};

use crate::{
    components::Header,
    config::{instance as config, Config},
};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct JournalEntry {
    pub title: String,
    pub date: String,
    pub body: String,
    pub subtitle: String,
}

pub fn page(cx: Scope) -> Element {
    let manifest = use_state(&cx, || None as Option<Vec<JournalEntryFragment>>);
    let base_url = web_sys::window().unwrap().origin();
    let journal_entry = use_state(&cx, || Ok(None) as Result<Option<JournalEntry>, ()>);
    let route = use_route(&cx);
    let router = use_router(&cx);

    let stem = route.segment("entry");
    if stem.is_none() {
        return cx.render(rsx!(Redirect { to: "/" }));
    }
    let stem = stem.unwrap();

    let base_url_cloned = base_url.clone();
    use_future(&cx, (manifest,), |(manifest,)| async move {
        if manifest.is_some() {
            return;
        }

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/static/entries.json", base_url_cloned))
            .send()
            .await
            .unwrap()
            .json::<Vec<JournalEntryFragment>>()
            .await
            .unwrap();

        manifest.set(Some(resp.clone()));
    });

    if let Some(manifest) = manifest.get() {
        if journal_entry.is_ok() {
            if let Some(item) = manifest
                .clone()
                .iter()
                .filter(|i| i.link == stem)
                .map(|i| i.clone())
                .collect::<Vec<JournalEntryFragment>>()
                .first()
            {
                let path = item.path.clone();
                let base_url = base_url.clone();
                use_future(&cx, (journal_entry,), |(journal_entry,)| async move {
                    let base_url = base_url.clone();
                    if journal_entry.is_err() || journal_entry.get().clone().unwrap().is_some() {
                        return;
                    }

                    let client = reqwest::Client::new();
                    let resp = client
                        .get(format!("{}/static/{}", base_url.to_owned(), path))
                        .send()
                        .await
                        .unwrap()
                        .json::<JournalEntry>()
                        .await;

                    if let Ok(resp) = resp {
                        journal_entry.set(Ok(Some(resp.clone())));
                    } else {
                        journal_entry.set(Err(()));
                    }
                });
            } else {
                return cx.render(rsx!(Redirect { to: "/" }));
            }
        }
    }

    if let Ok(entry) = journal_entry.get() {
        cx.render(rsx!(div {
            class: "w-full flex items-center flex-col mt-4 pb-12",
            div {
                class: "w-full flex items-center flex-col max-w-prose relative",
                div {
                    class: "absolute text-red-900 left-0 font-medium transition hover:opacity-75 cursor-pointer select-none",
                    onclick: |_| router.push_route("/", None, None),
                    "[back 2 home]"
                }
                JournalEntryHeader { entry: entry.clone() }
                JournalEntryBody { entry: entry.clone() }
            }
        }))
    } else {
        cx.render(rsx!(Redirect { to: "/" }))
    }
}
