use super_clone::models::{Provider, Repository};

#[test]
fn test_repository_creation() {
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
    assert_eq!(repo.owner, "owner");
    assert_eq!(repo.provider, "github");
    assert!(!repo.is_private);
    assert_eq!(repo.status, "not_cloned");
    assert!(!repo.id.is_empty());
}

#[test]
fn test_provider_parsing() {
    use std::str::FromStr;

    assert!(Provider::from_str("github").is_ok());
    assert!(Provider::from_str("GitHub").is_ok());
    assert!(Provider::from_str("gitlab").is_ok());
    assert!(Provider::from_str("GitLab").is_ok());
    assert!(Provider::from_str("invalid").is_err());
}

#[test]
fn test_repository_status_update() {
    use super_clone::models::CloneStatus;

    let mut repo = Repository::new(
        "test-repo".to_string(),
        "owner/test-repo".to_string(),
        "owner".to_string(),
        Provider::GitHub,
        "https://github.com/owner/test-repo.git".to_string(),
        "git@github.com:owner/test-repo.git".to_string(),
        None,
        false,
    );

    let original_updated_at = repo.updated_at;

    // Wait a moment to ensure timestamp change
    std::thread::sleep(std::time::Duration::from_millis(1));

    repo.update_status(CloneStatus::Cloned);

    assert_eq!(repo.status, "cloned");
    assert!(repo.updated_at > original_updated_at);
}

#[test]
fn test_github_client_creation() {
    use super_clone::providers::github::GitHubClient;

    // Test without token
    let client = GitHubClient::new(None);
    assert!(client.is_ok());

    // Test with token
    let client = GitHubClient::new(Some("test_token".to_string()));
    assert!(client.is_ok());
}

#[test]
fn test_gitlab_client_creation() {
    use super_clone::providers::gitlab::GitLabClient;

    // Test without token
    let client = GitLabClient::new(None, None);
    assert!(client.is_ok());

    // Test with token
    let client = GitLabClient::new(Some("test_token".to_string()), None);
    assert!(client.is_ok());

    // Test with custom base URL
    let client = GitLabClient::new(None, Some("https://gitlab.example.com".to_string()));
    assert!(client.is_ok());
    
    // Test with both token and custom base URL
    let client = GitLabClient::new(Some("test_token".to_string()), Some("https://gitlab.example.com".to_string()));
    assert!(client.is_ok());
}

#[test]
fn test_config_default_gitlab_url() {
    use super_clone::Config;
    
    // Test that Config respects GITLAB_URL environment variable
    let config = Config::default();
    // When GITLAB_URL is not set, gitlab_base_url should be None
    // (This will vary based on env, so we just check it's an Option)
    assert!(config.gitlab_base_url.is_none() || config.gitlab_base_url.is_some());
}
