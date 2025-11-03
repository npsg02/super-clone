# Quick Start Guide

Choose your preferred development method and get started in minutes!

## 🚀 Fastest Methods

### GitHub Codespaces (Zero Setup!)
1. Click "Code" → "Codespaces" → "Create codespace on main"
2. Wait ~2 minutes for setup
3. Start using super-clone!

**Best for:** Trying the project, contributing without local setup

---

### Docker Compose (Simple)
```bash
git clone https://github.com/npsg02/super-clone.git
cd super-clone
docker compose up
```

**Best for:** Quick testing, isolated environments

---

### Nix (Reproducible)
```bash
git clone https://github.com/npsg02/super-clone.git
cd super-clone
nix develop
cargo run
```

**Best for:** Guaranteed reproducible builds, NixOS users

---

### Local Development (Traditional)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/npsg02/super-clone.git
cd super-clone
cargo build --release
```

**Best for:** Full control, native performance

---

## 📋 Common Commands

### Using Make (Recommended)
```bash
make help          # Show all available commands
make build         # Build debug version
make test          # Run tests
make clippy        # Run linter
make fmt           # Format code
make all           # Format, lint, test, and build
```

### Using Cargo
```bash
cargo build                      # Debug build
cargo build --release            # Optimized build
cargo test                       # Run tests
cargo clippy -- -D warnings      # Lint
cargo fmt                        # Format
```

### Using Docker
```bash
docker build -t super-clone .                    # Build image
docker run --rm super-clone --help               # Show help
docker run --rm -it super-clone tui              # Interactive TUI
```

### Using Docker Compose
```bash
docker compose up                  # Start app
docker compose up dev              # Development mode
docker compose down                # Stop services
```

---

## 🎯 First Steps After Setup

1. **Run the application:**
   ```bash
   cargo run -- --help
   ```

2. **Try the TUI:**
   ```bash
   cargo run -- tui
   ```

3. **Clone repositories from a GitHub user:**
   ```bash
   # Set your GitHub token for private repos (optional)
   export GITHUB_TOKEN=your_token_here
   
   cargo run -- clone-user --provider github username
   ```

4. **Clone from a GitHub organization:**
   ```bash
   cargo run -- clone-org --provider github organization
   ```

5. **List all discovered repositories:**
   ```bash
   cargo run -- list
   ```

6. **Pull updates for all cloned repos:**
   ```bash
   cargo run -- pull-all
   ```

7. **Run tests:**
   ```bash
   cargo test
   ```

---

## 🔍 Need More Details?

- **Full setup instructions:** [SETUP.md](../SETUP.md)
- **Usage examples:** [README.md](../README.md)
- **CI/CD info:** [CI-CD.md](CI-CD.md)

---

## ❓ Troubleshooting

### "Git not found"
```bash
# Ubuntu/Debian
sudo apt-get install git

# macOS
brew install git

# Nix/Codespaces - Already included!
```

### "SQLite not found"
```bash
# Ubuntu/Debian
sudo apt-get install libsqlite3-dev

# macOS
brew install sqlite

# Nix/Codespaces - Already included!
```

### "OpenSSL not found"
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# macOS
brew install openssl

# Nix/Codespaces - Already included!
```

### "Slow first build"
This is normal! Cargo compiles all dependencies on first build.
Subsequent builds are much faster (seconds instead of minutes).

### Docker permission issues
```bash
# Create data directory with proper permissions
mkdir -p data repositories
chmod 777 data repositories
```

### API Rate Limits
- **GitHub**: 60 requests/hour (unauthenticated), 5000 requests/hour (authenticated)
- **GitLab**: 10 requests/second (authenticated)
- **Solution**: Use access tokens for better rate limits

---

## 🎓 Learning Resources

### Rust Basics
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Project-Specific
- Explore [src/](../src/) for code organization
- Check [tests/](../tests/) for testing examples
- Read [examples/](../examples/) for usage patterns

### Development Tools
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Rustfmt Configuration](https://rust-lang.github.io/rustfmt/)

---

## 💡 Tips

1. **Use rust-analyzer**: Best IDE support for Rust
2. **Run clippy often**: Catches bugs early
3. **Format before commit**: `cargo fmt` or `make fmt`
4. **Test frequently**: `cargo test` or `make test`
5. **Use `--release` for production**: Much faster
6. **Set access tokens**: For private repositories and better rate limits
7. **Use SSH**: For seamless authentication (configure SSH keys first)

---

## 🤝 Contributing

Ready to contribute?

1. Fork the repository
2. Create a branch: `git checkout -b feature/my-feature`
3. Make changes
4. Test: `make all` or `cargo test && cargo clippy && cargo fmt`
5. Commit: `git commit -m "Add feature"`
6. Push: `git push origin feature/my-feature`
7. Open a Pull Request

---

## 📞 Getting Help

- 📖 Read the [full docs](../README.md)
- 🐛 [Open an issue](https://github.com/npsg02/super-clone/issues)
- 💬 Check [existing issues](https://github.com/npsg02/super-clone/issues?q=is%3Aissue)

---

**Happy cloning! 🦀🚀**
