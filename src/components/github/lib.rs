use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct RepoUserNameNode {
    pub(crate) name: String,
    #[serde(alias = "isFork")]
    pub(crate) is_fork: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RepoUserName {
    pub(crate) nodes: Vec<RepoUserNameNode>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RepoUser {
    pub(crate) name: RepoUserName,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RepoResponseData {
    pub(crate) user: RepoUser,
}

#[derive(Deserialize, Debug)]
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

        for repo in self.data.user.name.nodes.iter().filter(|r| !r.is_fork) {
            log::info!("{}", repo.name);
            let resp = client
                .get(format!(
                    "https://api.github.com/repos/{}/{}/languages",
                    username, repo.name
                ))
                .bearer_auth(github_token.clone())
                .send()
                .await
                .unwrap()
                .json::<HashMap<String, i64>>()
                .await;

            if let Ok(resp) = resp {
                for (lang, bytes) in resp {
                    let current = languages.get(&lang).unwrap_or(&0);
                    languages.insert(lang, current + bytes);
                    total_bytes += bytes;
                }
            }
        }

        Some((languages, total_bytes))
    }
}
