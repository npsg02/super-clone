pub mod github;
pub mod gitlab;

use crate::models::Repository;
use crate::Result;

/// Trait for repository providers
#[async_trait::async_trait]
pub trait RepositoryProvider {
    /// Discover repositories for a user
    async fn discover_user_repos(&self, username: &str) -> Result<Vec<Repository>>;

    /// Discover repositories for an organization/group
    async fn discover_org_repos(&self, org: &str) -> Result<Vec<Repository>>;
}
