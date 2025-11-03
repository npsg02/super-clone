use super_clone::{
    database::RepositoryDatabase,
    models::{CloneStatus, Provider, Repository},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let db = RepositoryDatabase::new("example.db").await?;

    // Create some example repositories
    let repo1 = Repository::new(
        "super-clone".to_string(),
        "npsg02/super-clone".to_string(),
        "npsg02".to_string(),
        Provider::GitHub,
        "https://github.com/npsg02/super-clone.git".to_string(),
        "git@github.com:npsg02/super-clone.git".to_string(),
        Some("A tool to clone all repos from GitHub and GitLab".to_string()),
        false,
    );

    let repo2 = Repository::new(
        "rust".to_string(),
        "rust-lang/rust".to_string(),
        "rust-lang".to_string(),
        Provider::GitHub,
        "https://github.com/rust-lang/rust.git".to_string(),
        "git@github.com:rust-lang/rust.git".to_string(),
        Some("Empowering everyone to build reliable and efficient software".to_string()),
        false,
    );

    let repo3 = Repository::new(
        "awesome-project".to_string(),
        "group/awesome-project".to_string(),
        "group".to_string(),
        Provider::GitLab,
        "https://gitlab.com/group/awesome-project.git".to_string(),
        "git@gitlab.com:group/awesome-project.git".to_string(),
        Some("An awesome GitLab project".to_string()),
        true,
    );

    // Save repositories to database
    db.create_repository(&repo1).await?;
    db.create_repository(&repo2).await?;
    db.create_repository(&repo3).await?;

    // List all repositories
    println!("All repositories:");
    let repos = db.get_all_repositories().await?;
    for repo in &repos {
        let privacy = if repo.is_private { "🔒" } else { "  " };
        println!("  {} [{}] {}", privacy, repo.provider, repo.full_name);
        if let Some(description) = &repo.description {
            println!("    {}", description);
        }
    }

    // Update status of first repository to cloned
    let mut repo = repos[0].clone();
    repo.update_status(CloneStatus::Cloned);
    repo.set_local_path("/home/user/repositories/npsg02/super-clone".to_string());
    db.update_repository(&repo).await?;

    println!("\nAfter updating first repository:");
    let updated_repos = db.get_all_repositories().await?;
    for repo in &updated_repos {
        let status = match repo.status.as_str() {
            "cloned" => "✓",
            "not_cloned" => "○",
            _ => "?",
        };
        println!("  {} {}", status, repo.full_name);
        if let Some(path) = &repo.local_path {
            println!("    📁 {}", path);
        }
    }

    // Get only GitHub repositories
    println!("\nGitHub repositories:");
    let github_repos = db.get_repositories_by_provider("github").await?;
    for repo in &github_repos {
        println!("  {}", repo.full_name);
    }

    // Get cloned repositories
    println!("\nCloned repositories:");
    let cloned_repos = db.get_repositories_by_status(CloneStatus::Cloned).await?;
    for repo in &cloned_repos {
        println!("  {} -> {:?}", repo.full_name, repo.local_path);
    }

    Ok(())
}
