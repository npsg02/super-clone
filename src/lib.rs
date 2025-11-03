//! Super Clone - Repository Cloning Tool
//!
//! A CLI and TUI tool to clone and manage repositories from GitHub and GitLab.

pub mod database;
pub mod git;
pub mod models;
pub mod providers;
pub mod tui;

pub use models::*;

/// Application result type
pub type Result<T> = anyhow::Result<T>;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Database file path
    pub database_url: String,
    /// GitHub access token
    pub github_token: Option<String>,
    /// GitLab access token
    pub gitlab_token: Option<String>,
    /// Base path for cloning repositories
    pub clone_base_path: String,
    /// Use SSH for cloning (default: HTTPS)
    pub use_ssh: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".super-clone")
                .join("repositories.db")
                .to_string_lossy()
                .to_string(),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
            gitlab_token: std::env::var("GITLAB_TOKEN").ok(),
            clone_base_path: dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("repositories")
                .to_string_lossy()
                .to_string(),
            use_ssh: false,
        }
    }
}
