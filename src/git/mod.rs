use crate::models::Repository;
use crate::Result;
use anyhow::Context;
use std::path::PathBuf;
use std::process::Command;

pub struct GitOperations {
    base_path: PathBuf,
}

impl GitOperations {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Clone a repository
    pub async fn clone_repository(&self, repo: &Repository, use_ssh: bool) -> Result<String> {
        // Ensure base directory exists
        std::fs::create_dir_all(&self.base_path)?;

        // Determine clone URL
        let clone_url = if use_ssh {
            &repo.clone_url_ssh
        } else {
            &repo.clone_url_https
        };

        // Create target directory path
        let repo_path = self.base_path.join(&repo.owner).join(&repo.name);

        // Check if already exists
        if repo_path.exists() {
            // Check if it's a valid git repository
            let git_dir = repo_path.join(".git");
            if git_dir.exists() {
                return Ok(repo_path.to_string_lossy().to_string());
            } else {
                return Err(anyhow::anyhow!(
                    "Directory exists but is not a git repository: {}",
                    repo_path.display()
                ));
            }
        }

        // Ensure parent directory exists
        if let Some(parent) = repo_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Clone the repository
        let output = Command::new("git")
            .arg("clone")
            .arg(clone_url)
            .arg(&repo_path)
            .output()
            .context("Failed to execute git clone")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Git clone failed: {}", error));
        }

        Ok(repo_path.to_string_lossy().to_string())
    }

    /// Pull updates for a repository
    pub async fn pull_repository(&self, local_path: &str) -> Result<()> {
        let path = PathBuf::from(local_path);

        if !path.exists() {
            return Err(anyhow::anyhow!(
                "Repository path does not exist: {}",
                local_path
            ));
        }

        let output = Command::new("git")
            .arg("-C")
            .arg(&path)
            .arg("pull")
            .output()
            .context("Failed to execute git pull")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Git pull failed: {}", error));
        }

        Ok(())
    }

    /// Check if git is installed
    pub fn check_git_installed() -> Result<()> {
        let output = Command::new("git")
            .arg("--version")
            .output()
            .context("Failed to check git installation")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Git is not installed or not in PATH"));
        }

        Ok(())
    }

    /// Get the default clone path for a repository
    pub fn get_repo_path(&self, repo: &Repository) -> PathBuf {
        self.base_path.join(&repo.owner).join(&repo.name)
    }
}
