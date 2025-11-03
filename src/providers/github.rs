use crate::models::{Provider, Repository};
use crate::providers::RepositoryProvider;
use crate::Result;
use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitHubRepo {
    name: String,
    full_name: String,
    owner: GitHubOwner,
    clone_url: String,
    ssh_url: String,
    description: Option<String>,
    private: bool,
}

#[derive(Debug, Deserialize)]
struct GitHubOwner {
    login: String,
}

pub struct GitHubClient {
    client: reqwest::Client,
    #[allow(dead_code)]
    token: Option<String>,
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("super-clone/0.1.0"),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/vnd.github+json"),
        );

        if let Some(ref token) = token {
            let header_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))
                .context("Invalid GitHub token format")?;
            headers.insert(reqwest::header::AUTHORIZATION, header_value);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, token })
    }

    async fn fetch_repos(&self, url: &str) -> Result<Vec<Repository>> {
        let mut all_repos = Vec::new();
        let mut page = 1;
        let per_page = 100;

        loop {
            let url = format!("{}?page={}&per_page={}", url, page, per_page);
            let response = self
                .client
                .get(&url)
                .send()
                .await
                .context("Failed to fetch repositories from GitHub")?;

            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "GitHub API error: {} - {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ));
            }

            let repos: Vec<GitHubRepo> = response
                .json()
                .await
                .context("Failed to parse GitHub API response")?;

            if repos.is_empty() {
                break;
            }

            for repo in repos {
                all_repos.push(Repository::new(
                    repo.name,
                    repo.full_name,
                    repo.owner.login,
                    Provider::GitHub,
                    repo.clone_url,
                    repo.ssh_url,
                    repo.description,
                    repo.private,
                ));
            }

            page += 1;
        }

        Ok(all_repos)
    }
}

#[async_trait::async_trait]
impl RepositoryProvider for GitHubClient {
    async fn discover_user_repos(&self, username: &str) -> Result<Vec<Repository>> {
        let url = format!("https://api.github.com/users/{}/repos", username);
        self.fetch_repos(&url).await
    }

    async fn discover_org_repos(&self, org: &str) -> Result<Vec<Repository>> {
        let url = format!("https://api.github.com/orgs/{}/repos", org);
        self.fetch_repos(&url).await
    }
}
