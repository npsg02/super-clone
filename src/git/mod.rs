use crate::models::{Provider, Repository};
use crate::Result;
use anyhow::Context;
use std::path::PathBuf;
use std::process::Command;

pub struct GitOperations {
    base_path: PathBuf,
    github_token: Option<String>,
    gitlab_token: Option<String>,
}

impl GitOperations {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            github_token: None,
            gitlab_token: None,
        }
    }

    pub fn with_tokens(
        base_path: PathBuf,
        github_token: Option<String>,
        gitlab_token: Option<String>,
    ) -> Self {
        Self {
            base_path,
            github_token,
            gitlab_token,
        }
    }

    /// Inject authentication token into HTTPS URL for private repositories
    fn inject_token_into_url(&self, url: &str, provider: Provider, is_private: bool) -> String {
        // Only inject token for private repos using HTTPS
        if !is_private || !url.starts_with("https://") {
            return url.to_string();
        }

        let token = match provider {
            Provider::GitHub => self.github_token.as_ref(),
            Provider::GitLab => self.gitlab_token.as_ref(),
        };

        if let Some(token) = token {
            match provider {
                Provider::GitHub => {
                    // GitHub format: https://<token>@github.com/owner/repo.git
                    url.replace("https://", &format!("https://{}@", token))
                }
                Provider::GitLab => {
                    // GitLab format: https://oauth2:<token>@gitlab.com/owner/repo.git
                    url.replace("https://", &format!("https://oauth2:{}@", token))
                }
            }
        } else {
            url.to_string()
        }
    }

    /// Clone a repository
    pub async fn clone_repository(&self, repo: &Repository, use_ssh: bool) -> Result<String> {
        // Ensure base directory exists
        std::fs::create_dir_all(&self.base_path)?;

        // Determine clone URL
        let clone_url = if use_ssh {
            repo.clone_url_ssh.clone()
        } else {
            // For HTTPS, inject token if this is a private repository
            let provider: Provider = repo.provider.parse()?;
            self.inject_token_into_url(&repo.clone_url_https, provider, repo.is_private)
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
            .arg(&clone_url)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Provider;

    #[test]
    fn test_inject_token_github_private_repo() {
        let git_ops = GitOperations::with_tokens(
            PathBuf::from("/tmp"),
            Some("ghp_test_token_123".to_string()),
            None,
        );

        let url = "https://github.com/owner/private-repo.git";
        let result = git_ops.inject_token_into_url(url, Provider::GitHub, true);

        assert_eq!(
            result,
            "https://ghp_test_token_123@github.com/owner/private-repo.git"
        );
    }

    #[test]
    fn test_inject_token_github_public_repo() {
        let git_ops = GitOperations::with_tokens(
            PathBuf::from("/tmp"),
            Some("ghp_test_token_123".to_string()),
            None,
        );

        let url = "https://github.com/owner/public-repo.git";
        let result = git_ops.inject_token_into_url(url, Provider::GitHub, false);

        // Should not modify URL for public repos
        assert_eq!(result, "https://github.com/owner/public-repo.git");
    }

    #[test]
    fn test_inject_token_gitlab_private_repo() {
        let git_ops = GitOperations::with_tokens(
            PathBuf::from("/tmp"),
            None,
            Some("glpat_test_token_456".to_string()),
        );

        let url = "https://gitlab.com/group/private-project.git";
        let result = git_ops.inject_token_into_url(url, Provider::GitLab, true);

        assert_eq!(
            result,
            "https://oauth2:glpat_test_token_456@gitlab.com/group/private-project.git"
        );
    }

    #[test]
    fn test_inject_token_ssh_url_not_modified() {
        let git_ops = GitOperations::with_tokens(
            PathBuf::from("/tmp"),
            Some("ghp_test_token_123".to_string()),
            None,
        );

        let url = "git@github.com:owner/repo.git";
        let result = git_ops.inject_token_into_url(url, Provider::GitHub, true);

        // SSH URLs should not be modified
        assert_eq!(result, "git@github.com:owner/repo.git");
    }

    #[test]
    fn test_inject_token_no_token_provided() {
        let git_ops = GitOperations::with_tokens(PathBuf::from("/tmp"), None, None);

        let url = "https://github.com/owner/private-repo.git";
        let result = git_ops.inject_token_into_url(url, Provider::GitHub, true);

        // Should not modify URL when no token is provided
        assert_eq!(result, "https://github.com/owner/private-repo.git");
    }
}
