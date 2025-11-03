use crate::models::{Provider, Repository};
use crate::providers::RepositoryProvider;
use crate::Result;
use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitLabProject {
    name: String,
    path_with_namespace: String,
    namespace: GitLabNamespace,
    http_url_to_repo: String,
    ssh_url_to_repo: String,
    description: Option<String>,
    visibility: String,
}

#[derive(Debug, Deserialize)]
struct GitLabNamespace {
    path: String,
}

pub struct GitLabClient {
    client: reqwest::Client,
    #[allow(dead_code)]
    token: Option<String>,
    base_url: String,
}

impl GitLabClient {
    pub fn new(token: Option<String>, base_url: Option<String>) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("super-clone/0.1.0"),
        );

        if let Some(ref token) = token {
            let header_value = reqwest::header::HeaderValue::from_str(token)
                .context("Invalid GitLab token format")?;
            headers.insert(
                reqwest::header::HeaderName::from_static("private-token"),
                header_value,
            );
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            token,
            base_url: base_url.unwrap_or_else(|| "https://gitlab.com".to_string()),
        })
    }

    async fn fetch_projects(&self, url: &str) -> Result<Vec<Repository>> {
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
                .context("Failed to fetch projects from GitLab")?;

            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "GitLab API error: {} - {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ));
            }

            let projects: Vec<GitLabProject> = response
                .json()
                .await
                .context("Failed to parse GitLab API response")?;

            if projects.is_empty() {
                break;
            }

            for project in projects {
                let is_private = project.visibility != "public";
                all_repos.push(Repository::new(
                    project.name,
                    project.path_with_namespace.clone(),
                    project.namespace.path,
                    Provider::GitLab,
                    project.http_url_to_repo,
                    project.ssh_url_to_repo,
                    project.description,
                    is_private,
                ));
            }

            page += 1;
        }

        Ok(all_repos)
    }
}

#[async_trait::async_trait]
impl RepositoryProvider for GitLabClient {
    async fn discover_user_repos(&self, username: &str) -> Result<Vec<Repository>> {
        let url = format!("{}/api/v4/users/{}/projects", self.base_url, username);
        self.fetch_projects(&url).await
    }

    async fn discover_org_repos(&self, group: &str) -> Result<Vec<Repository>> {
        let url = format!("{}/api/v4/groups/{}/projects", self.base_url, group);
        self.fetch_projects(&url).await
    }
}
