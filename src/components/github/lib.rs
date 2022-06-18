use dioxus::{prelude::*, router::*};
use futures::{stream, StreamExt, TryStreamExt};
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct Color {
    pub(crate) color: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RepoUserNameNode {
    pub(crate) name: String,
    #[serde(alias = "isArchived")]
    pub(crate) is_archived: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RepoRepos {
    pub(crate) nodes: Vec<RepoUserNameNode>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RepoUser {
    pub(crate) repos: RepoRepos,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RepoResponseData {
    pub(crate) user: RepoUser,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RepoResponse {
    pub(crate) data: RepoResponseData,
}

pub(crate) type LanguageList = (HashMap<String, i64>, i64);

impl RepoResponse {
    pub(crate) async fn get_languages<U: Into<String>, G: Into<String>>(
        self,
        username: U,
        github_token: G,
    ) -> Option<LanguageList> {
        let client = reqwest::Client::new();
        let mut total_bytes = 0;
        let username = username.into();
        let github_token = github_token.into();
        let mut languages: HashMap<String, i64> = HashMap::new();

        let urls = self
            .data
            .user
            .repos
            .nodes
            .iter()
            .filter(|r| !r.is_archived)
            .map(|repo| {
                format!(
                    "https://api.github.com/repos/{}/{}/languages",
                    username, repo.name
                )
            });

        let results = stream::iter(urls)
            .map(|url| {
                let client = &client;
                let github_token = &github_token;
                async move {
                    if let Ok(req) = client.get(url).bearer_auth(github_token).send().await {
                        if let Ok(res) = req.json().await {
                            return res;
                        }
                    }

                    return HashMap::new();
                }
            })
            .buffer_unordered(10)
            .collect::<Vec<HashMap<String, i64>>>()
            .await;

        for res in results {
            for (lang, bytes) in res {
                let current = languages.get(&lang).unwrap_or(&0);
                languages.insert(lang, current + bytes);
                total_bytes += bytes;
            }
        }

        log::info!("{:?}", languages);

        Some((languages, total_bytes))
    }
}
