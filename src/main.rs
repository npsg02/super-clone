use clap::{Parser, Subcommand};
use std::path::PathBuf;
use super_clone::{
    database::RepositoryDatabase,
    git::GitOperations,
    models::{CloneStatus, Provider},
    providers::{github::GitHubClient, gitlab::GitLabClient, RepositoryProvider},
    tui::App,
    Config,
};

/// A CLI and TUI tool to clone and manage repositories from GitHub and GitLab
#[derive(Parser)]
#[command(name = "super-clone")]
#[command(about = "Clone and manage repositories from GitHub and GitLab")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Database file path
    #[arg(short, long)]
    database: Option<String>,

    /// GitHub access token (or set GITHUB_TOKEN env var)
    #[arg(long)]
    github_token: Option<String>,

    /// GitLab access token (or set GITLAB_TOKEN env var)
    #[arg(long)]
    gitlab_token: Option<String>,

    /// Base path for cloning repositories
    #[arg(short = 'p', long)]
    clone_path: Option<String>,

    /// Use SSH for cloning (default: HTTPS)
    #[arg(long)]
    ssh: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the interactive TUI
    Tui,

    /// Clone all repositories from a GitHub user
    CloneUser {
        /// Provider (github or gitlab)
        #[arg(short, long, default_value = "github")]
        provider: String,
        /// Username
        username: String,
    },

    /// Clone all repositories from a GitHub organization or GitLab group
    CloneOrg {
        /// Provider (github or gitlab)
        #[arg(short, long, default_value = "github")]
        provider: String,
        /// Organization or group name
        org: String,
    },

    /// Clone all repositories for the authenticated user (requires token)
    CloneMine {
        /// Provider (github or gitlab)
        #[arg(short, long, default_value = "github")]
        provider: String,
    },

    /// Clone all repositories from all organizations/groups the authenticated user has access to
    CloneAllOrgs {
        /// Provider (github or gitlab)
        #[arg(short, long, default_value = "github")]
        provider: String,
    },

    /// List discovered repositories
    List {
        /// Filter by provider (github or gitlab)
        #[arg(short, long)]
        provider: Option<String>,
        /// Show only cloned repositories
        #[arg(short, long)]
        cloned: bool,
    },

    /// Pull updates for all cloned repositories
    PullAll,

    /// Clone a specific repository by full name
    Clone {
        /// Repository full name (e.g., owner/repo)
        repo: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Create config
    let mut config = Config::default();
    if let Some(db) = cli.database {
        config.database_url = db;
    }
    config.github_token = cli
        .github_token
        .or_else(|| std::env::var("GITHUB_TOKEN").ok());
    config.gitlab_token = cli
        .gitlab_token
        .or_else(|| std::env::var("GITLAB_TOKEN").ok());
    if let Some(path) = cli.clone_path {
        config.clone_base_path = path;
    }
    config.use_ssh = cli.ssh;

    // Check if git is installed
    GitOperations::check_git_installed()?;

    let db = RepositoryDatabase::new(&config.database_url).await?;

    match cli.command {
        Some(Commands::Tui) | None => {
            // Default to TUI mode
            let mut app = App::new(db);
            app.run().await?;
        }
        Some(Commands::CloneUser { provider, username }) => {
            let provider_enum: Provider = provider.parse()?;

            println!("🔍 Discovering repositories for user: {}", username);
            let repos = match provider_enum {
                Provider::GitHub => {
                    let client = GitHubClient::new(config.github_token)?;
                    client.discover_user_repos(&username).await?
                }
                Provider::GitLab => {
                    let client = GitLabClient::new(config.gitlab_token, None)?;
                    client.discover_user_repos(&username).await?
                }
            };

            println!("📦 Found {} repositories", repos.len());

            // Save to database
            for repo in &repos {
                if db
                    .get_repository_by_full_name(&repo.full_name)
                    .await?
                    .is_none()
                {
                    db.create_repository(repo).await?;
                }
            }

            // Clone repositories
            let git_ops = GitOperations::new(PathBuf::from(&config.clone_base_path));
            for repo in &repos {
                println!("⬇️  Cloning: {}", repo.full_name);
                match git_ops.clone_repository(repo, config.use_ssh).await {
                    Ok(path) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.set_local_path(path.clone());
                        updated_repo.update_status(CloneStatus::Cloned);
                        db.update_repository(&updated_repo).await?;
                        println!("   ✅ Cloned to: {}", path);
                    }
                    Err(e) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.update_status(CloneStatus::Error);
                        db.update_repository(&updated_repo).await?;
                        eprintln!("   ❌ Failed: {}", e);
                    }
                }
            }

            println!("✨ Done!");
        }
        Some(Commands::CloneOrg { provider, org }) => {
            let provider_enum: Provider = provider.parse()?;

            println!(
                "🔍 Discovering repositories for organization/group: {}",
                org
            );
            let repos = match provider_enum {
                Provider::GitHub => {
                    let client = GitHubClient::new(config.github_token)?;
                    client.discover_org_repos(&org).await?
                }
                Provider::GitLab => {
                    let client = GitLabClient::new(config.gitlab_token, None)?;
                    client.discover_org_repos(&org).await?
                }
            };

            println!("📦 Found {} repositories", repos.len());

            // Save to database
            for repo in &repos {
                if db
                    .get_repository_by_full_name(&repo.full_name)
                    .await?
                    .is_none()
                {
                    db.create_repository(repo).await?;
                }
            }

            // Clone repositories
            let git_ops = GitOperations::new(PathBuf::from(&config.clone_base_path));
            for repo in &repos {
                println!("⬇️  Cloning: {}", repo.full_name);
                match git_ops.clone_repository(repo, config.use_ssh).await {
                    Ok(path) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.set_local_path(path.clone());
                        updated_repo.update_status(CloneStatus::Cloned);
                        db.update_repository(&updated_repo).await?;
                        println!("   ✅ Cloned to: {}", path);
                    }
                    Err(e) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.update_status(CloneStatus::Error);
                        db.update_repository(&updated_repo).await?;
                        eprintln!("   ❌ Failed: {}", e);
                    }
                }
            }

            println!("✨ Done!");
        }
        Some(Commands::CloneMine { provider }) => {
            let provider_enum: Provider = provider.parse()?;

            // Check for token first
            match provider_enum {
                Provider::GitHub => {
                    if config.github_token.is_none() {
                        return Err(anyhow::anyhow!(
                            "GitHub token is required for this command. Set GITHUB_TOKEN env var or use --github-token flag."
                        ).into());
                    }
                }
                Provider::GitLab => {
                    if config.gitlab_token.is_none() {
                        return Err(anyhow::anyhow!(
                            "GitLab token is required for this command. Set GITLAB_TOKEN env var or use --gitlab-token flag."
                        ).into());
                    }
                }
            }

            // Get authenticated user and discover repos
            println!("🔍 Discovering repositories for authenticated user...");
            let repos = match provider_enum {
                Provider::GitHub => {
                    let client = GitHubClient::new(config.github_token)?;
                    let username = client.get_authenticated_user().await?;
                    println!("   Authenticated as: {}", username);
                    client.discover_user_repos(&username).await?
                }
                Provider::GitLab => {
                    let client = GitLabClient::new(config.gitlab_token, None)?;
                    let username = client.get_authenticated_user().await?;
                    println!("   Authenticated as: {}", username);
                    client.discover_user_repos(&username).await?
                }
            };

            println!("📦 Found {} repositories", repos.len());

            // Save to database
            for repo in &repos {
                if db
                    .get_repository_by_full_name(&repo.full_name)
                    .await?
                    .is_none()
                {
                    db.create_repository(repo).await?;
                }
            }

            // Clone repositories
            let git_ops = GitOperations::new(PathBuf::from(&config.clone_base_path));
            for repo in &repos {
                println!("⬇️  Cloning: {}", repo.full_name);
                match git_ops.clone_repository(repo, config.use_ssh).await {
                    Ok(path) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.set_local_path(path.clone());
                        updated_repo.update_status(CloneStatus::Cloned);
                        db.update_repository(&updated_repo).await?;
                        println!("   ✅ Cloned to: {}", path);
                    }
                    Err(e) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.update_status(CloneStatus::Error);
                        db.update_repository(&updated_repo).await?;
                        eprintln!("   ❌ Failed: {}", e);
                    }
                }
            }

            println!("✨ Done!");
        }
        Some(Commands::CloneAllOrgs { provider }) => {
            let provider_enum: Provider = provider.parse()?;

            // Check for token first
            match provider_enum {
                Provider::GitHub => {
                    if config.github_token.is_none() {
                        return Err(anyhow::anyhow!(
                            "GitHub token is required for this command. Set GITHUB_TOKEN env var or use --github-token flag."
                        ).into());
                    }
                }
                Provider::GitLab => {
                    if config.gitlab_token.is_none() {
                        return Err(anyhow::anyhow!(
                            "GitLab token is required for this command. Set GITLAB_TOKEN env var or use --gitlab-token flag."
                        ).into());
                    }
                }
            }

            println!("🔍 Discovering organizations/groups...");
            
            let mut all_repos = Vec::new();
            match provider_enum {
                Provider::GitHub => {
                    let client = GitHubClient::new(config.github_token.clone())?;
                    let orgs = client.get_user_organizations().await?;
                    println!("   Found {} organizations with access", orgs.len());
                    
                    for org in orgs {
                        println!("   Discovering repositories for: {}", org);
                        let repos = client.discover_org_repos(&org).await?;
                        println!("   📦 Found {} repositories in {}", repos.len(), org);
                        all_repos.extend(repos);
                    }
                }
                Provider::GitLab => {
                    let client = GitLabClient::new(config.gitlab_token.clone(), None)?;
                    let orgs = client.get_user_groups().await?;
                    println!("   Found {} groups with access", orgs.len());
                    
                    for org in orgs {
                        println!("   Discovering repositories for: {}", org);
                        let repos = client.discover_org_repos(&org).await?;
                        println!("   📦 Found {} repositories in {}", repos.len(), org);
                        all_repos.extend(repos);
                    }
                }
            }

            println!("📦 Total: {} repositories across all organizations/groups", all_repos.len());

            // Save to database
            for repo in &all_repos {
                if db
                    .get_repository_by_full_name(&repo.full_name)
                    .await?
                    .is_none()
                {
                    db.create_repository(repo).await?;
                }
            }

            // Clone repositories
            let git_ops = GitOperations::new(PathBuf::from(&config.clone_base_path));
            for repo in &all_repos {
                println!("⬇️  Cloning: {}", repo.full_name);
                match git_ops.clone_repository(repo, config.use_ssh).await {
                    Ok(path) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.set_local_path(path.clone());
                        updated_repo.update_status(CloneStatus::Cloned);
                        db.update_repository(&updated_repo).await?;
                        println!("   ✅ Cloned to: {}", path);
                    }
                    Err(e) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.update_status(CloneStatus::Error);
                        db.update_repository(&updated_repo).await?;
                        eprintln!("   ❌ Failed: {}", e);
                    }
                }
            }

            println!("✨ Done!");
        }
        Some(Commands::List { provider, cloned }) => {
            let repos = if let Some(p) = provider {
                db.get_repositories_by_provider(&p).await?
            } else if cloned {
                db.get_repositories_by_status(CloneStatus::Cloned).await?
            } else {
                db.get_all_repositories().await?
            };

            if repos.is_empty() {
                println!("No repositories found.");
            } else {
                println!("📦 Repositories ({})", repos.len());
                for repo in repos {
                    let status = match repo.status.as_str() {
                        "cloned" => "✓",
                        "not_cloned" => "○",
                        "error" => "✗",
                        _ => "?",
                    };
                    let privacy = if repo.is_private { "🔒" } else { "  " };
                    println!(
                        "{} {} [{}] {}",
                        status, privacy, repo.provider, repo.full_name
                    );
                    if let Some(path) = &repo.local_path {
                        println!("   📁 {}", path);
                    }
                }
            }
        }
        Some(Commands::PullAll) => {
            let repos = db.get_repositories_by_status(CloneStatus::Cloned).await?;

            if repos.is_empty() {
                println!("No cloned repositories found.");
                return Ok(());
            }

            println!("🔄 Pulling updates for {} repositories", repos.len());

            let git_ops = GitOperations::new(PathBuf::from(&config.clone_base_path));
            for repo in repos {
                if let Some(path) = &repo.local_path {
                    print!("⬇️  Pulling: {} ... ", repo.full_name);
                    match git_ops.pull_repository(path).await {
                        Ok(_) => {
                            let mut updated_repo = repo.clone();
                            updated_repo.update_pulled_at();
                            db.update_repository(&updated_repo).await?;
                            println!("✅");
                        }
                        Err(e) => {
                            println!("❌ Failed: {}", e);
                        }
                    }
                }
            }

            println!("✨ Done!");
        }
        Some(Commands::Clone { repo }) => {
            let repository = db.get_repository_by_full_name(&repo).await?;

            if let Some(repo) = repository {
                let git_ops = GitOperations::new(PathBuf::from(&config.clone_base_path));
                println!("⬇️  Cloning: {}", repo.full_name);
                match git_ops.clone_repository(&repo, config.use_ssh).await {
                    Ok(path) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.set_local_path(path.clone());
                        updated_repo.update_status(CloneStatus::Cloned);
                        db.update_repository(&updated_repo).await?;
                        println!("✅ Cloned to: {}", path);
                    }
                    Err(e) => {
                        let mut updated_repo = repo.clone();
                        updated_repo.update_status(CloneStatus::Error);
                        db.update_repository(&updated_repo).await?;
                        eprintln!("❌ Failed: {}", e);
                    }
                }
            } else {
                eprintln!("Repository not found: {}", repo);
                eprintln!("First discover it using 'clone-user' or 'clone-org' command");
            }
        }
    }

    Ok(())
}
