use crate::models::{CloneStatus, Repository};
use crate::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

/// Database service for repository operations
#[derive(Debug, Clone)]
pub struct RepositoryDatabase {
    pool: SqlitePool,
}

impl RepositoryDatabase {
    /// Create a new database connection
    pub async fn new(database_url: &str) -> Result<Self> {
        // Handle special cases for SQLite URL format
        let url = match database_url {
            ":memory:" => "sqlite::memory:".to_string(),
            path if path.starts_with("sqlite://") => path.to_string(),
            path => {
                // Create parent directory if needed for file databases
                if let Some(parent) = std::path::Path::new(path).parent() {
                    std::fs::create_dir_all(parent)?;
                }
                format!("sqlite://{}?mode=rwc", path)
            }
        };

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;

        let db = Self { pool };
        db.migrate().await?;
        Ok(db)
    }

    /// Run database migrations
    async fn migrate(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS repositories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                full_name TEXT NOT NULL,
                owner TEXT NOT NULL,
                provider TEXT NOT NULL,
                clone_url_https TEXT NOT NULL,
                clone_url_ssh TEXT NOT NULL,
                description TEXT,
                is_private BOOLEAN NOT NULL DEFAULT FALSE,
                local_path TEXT,
                status TEXT NOT NULL DEFAULT 'not_cloned',
                last_pulled_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Get all repositories
    pub async fn get_all_repositories(&self) -> Result<Vec<Repository>> {
        let repos =
            sqlx::query_as::<_, Repository>("SELECT * FROM repositories ORDER BY full_name ASC")
                .fetch_all(&self.pool)
                .await?;
        Ok(repos)
    }

    /// Get repositories by provider
    pub async fn get_repositories_by_provider(&self, provider: &str) -> Result<Vec<Repository>> {
        let repos = sqlx::query_as::<_, Repository>(
            "SELECT * FROM repositories WHERE provider = ? ORDER BY full_name ASC",
        )
        .bind(provider)
        .fetch_all(&self.pool)
        .await?;
        Ok(repos)
    }

    /// Get repositories by owner
    pub async fn get_repositories_by_owner(&self, owner: &str) -> Result<Vec<Repository>> {
        let repos = sqlx::query_as::<_, Repository>(
            "SELECT * FROM repositories WHERE owner = ? ORDER BY full_name ASC",
        )
        .bind(owner)
        .fetch_all(&self.pool)
        .await?;
        Ok(repos)
    }

    /// Get repositories by status
    pub async fn get_repositories_by_status(&self, status: CloneStatus) -> Result<Vec<Repository>> {
        let repos = sqlx::query_as::<_, Repository>(
            "SELECT * FROM repositories WHERE status = ? ORDER BY full_name ASC",
        )
        .bind(status.to_string())
        .fetch_all(&self.pool)
        .await?;
        Ok(repos)
    }

    /// Get a repository by ID
    pub async fn get_repository(&self, id: &str) -> Result<Option<Repository>> {
        let repo = sqlx::query_as::<_, Repository>("SELECT * FROM repositories WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(repo)
    }

    /// Get a repository by full name
    pub async fn get_repository_by_full_name(&self, full_name: &str) -> Result<Option<Repository>> {
        let repo =
            sqlx::query_as::<_, Repository>("SELECT * FROM repositories WHERE full_name = ?")
                .bind(full_name)
                .fetch_optional(&self.pool)
                .await?;
        Ok(repo)
    }

    /// Create a new repository
    pub async fn create_repository(&self, repo: &Repository) -> Result<()> {
        sqlx::query(
            "INSERT INTO repositories (id, name, full_name, owner, provider, clone_url_https, clone_url_ssh, description, is_private, local_path, status, last_pulled_at, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&repo.id)
        .bind(&repo.name)
        .bind(&repo.full_name)
        .bind(&repo.owner)
        .bind(&repo.provider)
        .bind(&repo.clone_url_https)
        .bind(&repo.clone_url_ssh)
        .bind(&repo.description)
        .bind(repo.is_private)
        .bind(&repo.local_path)
        .bind(&repo.status)
        .bind(repo.last_pulled_at.map(|dt| dt.to_rfc3339()))
        .bind(repo.created_at.to_rfc3339())
        .bind(repo.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Update a repository
    pub async fn update_repository(&self, repo: &Repository) -> Result<()> {
        sqlx::query(
            "UPDATE repositories SET name = ?, full_name = ?, owner = ?, provider = ?, clone_url_https = ?, clone_url_ssh = ?, description = ?, is_private = ?, local_path = ?, status = ?, last_pulled_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&repo.name)
        .bind(&repo.full_name)
        .bind(&repo.owner)
        .bind(&repo.provider)
        .bind(&repo.clone_url_https)
        .bind(&repo.clone_url_ssh)
        .bind(&repo.description)
        .bind(repo.is_private)
        .bind(&repo.local_path)
        .bind(&repo.status)
        .bind(repo.last_pulled_at.map(|dt| dt.to_rfc3339()))
        .bind(repo.updated_at.to_rfc3339())
        .bind(&repo.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Delete a repository
    pub async fn delete_repository(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM repositories WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Clear all repositories (useful for refresh operations)
    pub async fn clear_all_repositories(&self) -> Result<()> {
        sqlx::query("DELETE FROM repositories")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
