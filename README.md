# Super Clone

A powerful CLI and TUI tool to clone and manage repositories from GitHub and GitLab.

## Features

- 🚀 Clone all repositories from GitHub users and organizations
- 🦊 Clone all repositories from GitLab users and groups
- 🔍 Auto-discover repositories from users, organizations, and groups
- 🔄 Pull updates for all cloned repositories with a single command
- 🔑 Support for both SSH and HTTPS cloning
- 🔒 Works with private repositories using access tokens
- 🖥️ Interactive Terminal User Interface (TUI)
- 🔧 Command Line Interface (CLI)
- 💾 SQLite database to track repository status
- 🧪 Comprehensive test suite
- 🚀 CI/CD with GitHub Actions
- 📦 Cross-platform releases
- 🔒 Security auditing
- 🐳 Docker and Docker Compose support
- ❄️ Nix flakes for reproducible environments
- 📦 Devcontainer configuration for GitHub Codespaces

## Installation

> **💡 Quick Start**: See [SETUP.md](SETUP.md) for detailed setup instructions using Docker, Nix, Codespaces, or local development.

### From Source

```bash
git clone https://github.com/npsg02/super-clone.git
cd super-clone
cargo build --release
```

### From Releases

Download the latest binary from the [Releases](https://github.com/npsg02/super-clone/releases) page.

### With Docker

```bash
# Build the image
docker build -t super-clone:latest .

# Run with interactive TUI
docker run --rm -it -v $(pwd)/data:/app/data super-clone:latest tui

# Or use Docker Compose
docker compose up
```

### With Nix

```bash
# Enter development environment
nix develop

# Or run directly
nix run
```

### With GitHub Codespaces

Click the "Code" button on GitHub and select "Create codespace on main" - everything is pre-configured!

## Usage

### Command Line Interface

```bash
# Show help
./super-clone --help

# Clone all repositories from a GitHub user
./super-clone clone-user --provider github username

# Clone all repositories from a GitHub organization
./super-clone clone-org --provider github organization-name

# Clone all repositories from a GitLab user
./super-clone clone-user --provider gitlab username

# Clone all repositories from a GitLab group
./super-clone clone-org --provider gitlab group-name

# List all discovered repositories
./super-clone list

# List only cloned repositories
./super-clone list --cloned

# List only GitHub repositories
./super-clone list --provider github

# Pull updates for all cloned repositories
./super-clone pull-all

# Clone a specific repository (must be discovered first)
./super-clone clone owner/repo

# Start interactive TUI (default mode)
./super-clone tui
```

### Using Access Tokens

For private repositories, set environment variables or use command-line flags:

```bash
# Using environment variables
export GITHUB_TOKEN=your_github_token
export GITLAB_TOKEN=your_gitlab_token
./super-clone clone-user --provider github username

# Using command-line flags
./super-clone --github-token your_token clone-user --provider github username
```

### SSH vs HTTPS

By default, super-clone uses HTTPS for cloning. To use SSH:

```bash
./super-clone --ssh clone-user --provider github username
```

### Custom Clone Path

Specify a custom base path for cloning repositories:

```bash
./super-clone --clone-path /path/to/repos clone-user --provider github username
```

### Terminal User Interface (TUI)

Start the interactive mode:

```bash
./super-clone tui
```

#### TUI Commands:
- `h` - Show help
- `q` - Quit application
- `d` - Delete selected repository
- `r` - Refresh repository list
- `a` - Show all repositories
- `g` - Show GitHub repositories only
- `l` - Show GitLab repositories only
- `c` - Show cloned repositories only
- `n` - Show not cloned repositories only
- `↑↓` - Navigate repositories

## Project Structure

```
super-clone/
├── .github/workflows/    # CI/CD workflows
├── src/
│   ├── database/         # Database layer
│   ├── models/           # Data models (Repository, Provider, Config)
│   ├── providers/        # GitHub and GitLab API clients
│   ├── git/              # Git operations (clone, pull)
│   ├── tui/              # Terminal UI
│   ├── lib.rs            # Library root
│   └── main.rs           # CLI application
├── tests/                # Integration tests
├── docs/                 # Documentation
└── examples/             # Usage examples
```

## Development

> **📚 Full Setup Guide**: See [SETUP.md](SETUP.md) for comprehensive development environment setup instructions.

### Prerequisites

Choose your preferred development method:

- **Local**: Rust 1.70 or later, SQLite3, Git
- **Docker**: Docker 20.10+ and Docker Compose
- **Nix**: Nix package manager with flakes enabled
- **Codespaces**: Just a GitHub account!

### Building

```bash
# Local
cargo build

# Docker
docker compose up --build

# Nix
nix build
```

### Running Tests

```bash
cargo test
```

### Running Clippy (Linter)

```bash
cargo clippy -- -D warnings
```

### Formatting Code

```bash
cargo fmt
```

### Development Environments

The project provides multiple development environment options:

- **Docker Compose**: `docker compose up dev` - Containerized development with live code mounting
- **Nix Flakes**: `nix develop` - Reproducible environment with all dependencies
- **Devcontainer**: Open in VS Code or GitHub Codespaces - Fully configured IDE
- **Traditional**: Local Rust installation with cargo

## Database

The application uses SQLite for persistence. By default, it creates a database at `~/.super-clone/repositories.db`. You can specify a different database path:

```bash
./super-clone --database /path/to/your/repos.db list
```

## Configuration

Super-clone stores configuration in the database and uses environment variables for sensitive data:

- `GITHUB_TOKEN`: GitHub personal access token for private repositories
- `GITLAB_TOKEN`: GitLab personal access token for private repositories

Default clone path: `~/repositories`

## CI/CD

The project includes comprehensive GitHub Actions workflows:

- **CI** (`ci.yml`): Build, test, lint, and format checks on multiple platforms (Linux, macOS, Windows)
- **Security** (`security.yml`): Weekly security audits with `cargo audit`
- **Release** (`release.yml`): Automated binary releases for Linux, macOS, and Windows on version tags
- **Docker** (`docker.yml`): Docker image build testing and docker-compose validation

All workflows run automatically on push and pull requests to ensure code quality and security.

## API Rate Limits

Be aware of API rate limits:

- **GitHub**: 60 requests/hour (unauthenticated), 5000 requests/hour (authenticated)
- **GitLab**: 10 requests/second (authenticated)

Using access tokens is recommended for better rate limits and access to private repositories.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
