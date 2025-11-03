use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Repository provider type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    GitHub,
    GitLab,
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::GitHub => write!(f, "github"),
            Provider::GitLab => write!(f, "gitlab"),
        }
    }
}

impl std::str::FromStr for Provider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(Provider::GitHub),
            "gitlab" => Ok(Provider::GitLab),
            _ => Err(anyhow::anyhow!("Invalid provider: {}", s)),
        }
    }
}

/// Repository clone status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CloneStatus {
    NotCloned,
    Cloning,
    Cloned,
    UpdateAvailable,
    Updating,
    Error,
}

impl std::fmt::Display for CloneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloneStatus::NotCloned => write!(f, "not_cloned"),
            CloneStatus::Cloning => write!(f, "cloning"),
            CloneStatus::Cloned => write!(f, "cloned"),
            CloneStatus::UpdateAvailable => write!(f, "update_available"),
            CloneStatus::Updating => write!(f, "updating"),
            CloneStatus::Error => write!(f, "error"),
        }
    }
}

/// Repository model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub provider: String,
    pub clone_url_https: String,
    pub clone_url_ssh: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub local_path: Option<String>,
    pub status: String,
    pub last_pulled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Repository {
    /// Create a new repository record
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        full_name: String,
        owner: String,
        provider: Provider,
        clone_url_https: String,
        clone_url_ssh: String,
        description: Option<String>,
        is_private: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            full_name,
            owner,
            provider: provider.to_string(),
            clone_url_https,
            clone_url_ssh,
            description,
            is_private,
            local_path: None,
            status: CloneStatus::NotCloned.to_string(),
            last_pulled_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update repository status
    pub fn update_status(&mut self, status: CloneStatus) {
        self.status = status.to_string();
        self.updated_at = Utc::now();
    }

    /// Set local path after cloning
    pub fn set_local_path(&mut self, path: String) {
        self.local_path = Some(path);
        self.updated_at = Utc::now();
    }

    /// Update last pulled timestamp
    pub fn update_pulled_at(&mut self) {
        self.last_pulled_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

/// Configuration model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub github_token: Option<String>,
    pub gitlab_token: Option<String>,
    pub clone_base_path: String,
    pub use_ssh: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            github_token: None,
            gitlab_token: None,
            clone_base_path: dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("repositories")
                .to_string_lossy()
                .to_string(),
            use_ssh: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_repository() {
        let repo = Repository::new(
            "test-repo".to_string(),
            "owner/test-repo".to_string(),
            "owner".to_string(),
            Provider::GitHub,
            "https://github.com/owner/test-repo.git".to_string(),
            "git@github.com:owner/test-repo.git".to_string(),
            Some("Test repository".to_string()),
            false,
        );
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.full_name, "owner/test-repo");
        assert_eq!(repo.provider, "github");
        assert!(!repo.is_private);
        assert_eq!(repo.status, "not_cloned");
    }

    #[test]
    fn test_provider_to_string() {
        assert_eq!(Provider::GitHub.to_string(), "github");
        assert_eq!(Provider::GitLab.to_string(), "gitlab");
    }

    #[test]
    fn test_provider_from_str() {
        assert_eq!("github".parse::<Provider>().unwrap(), Provider::GitHub);
        assert_eq!("GitLab".parse::<Provider>().unwrap(), Provider::GitLab);
    }
}
