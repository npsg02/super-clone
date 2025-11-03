use crate::database::RepositoryDatabase;
use crate::models::{CloneStatus, Repository};
use crate::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use tokio::time::Duration;

/// Application state
pub struct App {
    db: RepositoryDatabase,
    repos: Vec<Repository>,
    selected: ListState,
    status_message: String,
    filter: Filter,
}

#[derive(Debug, Clone)]
pub enum Filter {
    All,
    GitHub,
    GitLab,
    Cloned,
    NotCloned,
}

impl App {
    pub fn new(db: RepositoryDatabase) -> Self {
        let mut selected = ListState::default();
        selected.select(Some(0));

        Self {
            db,
            repos: Vec::new(),
            selected,
            status_message: "Welcome to Super Clone! Press 'h' for help.".to_string(),
            filter: Filter::All,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        self.refresh_repos().await?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && self.handle_input(key.code).await? {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_input(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('h') => {
                self.status_message = "Commands: q=quit, d=delete, r=refresh, g=GitHub only, l=GitLab only, a=all, c=cloned, n=not cloned, ↑↓=navigate".to_string();
            }
            KeyCode::Char('d') => {
                if let Some(index) = self.selected.selected() {
                    if index < self.repos.len() {
                        let repo = &self.repos[index];
                        self.db.delete_repository(&repo.id).await?;
                        self.refresh_repos().await?;
                        self.status_message = "Repository deleted!".to_string();
                    }
                }
            }
            KeyCode::Char('r') => {
                self.refresh_repos().await?;
                self.status_message = "Repositories refreshed!".to_string();
            }
            KeyCode::Char('a') => {
                self.filter = Filter::All;
                self.refresh_repos().await?;
                self.status_message = "Showing all repositories".to_string();
            }
            KeyCode::Char('g') => {
                self.filter = Filter::GitHub;
                self.refresh_repos().await?;
                self.status_message = "Showing GitHub repositories".to_string();
            }
            KeyCode::Char('l') => {
                self.filter = Filter::GitLab;
                self.refresh_repos().await?;
                self.status_message = "Showing GitLab repositories".to_string();
            }
            KeyCode::Char('c') => {
                self.filter = Filter::Cloned;
                self.refresh_repos().await?;
                self.status_message = "Showing cloned repositories".to_string();
            }
            KeyCode::Char('n') => {
                self.filter = Filter::NotCloned;
                self.refresh_repos().await?;
                self.status_message = "Showing not cloned repositories".to_string();
            }
            KeyCode::Down => {
                let i = match self.selected.selected() {
                    Some(i) => {
                        if i >= self.repos.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.selected.select(Some(i));
            }
            KeyCode::Up => {
                let i = match self.selected.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.repos.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.selected.select(Some(i));
            }
            _ => {}
        }
        Ok(false)
    }

    async fn refresh_repos(&mut self) -> Result<()> {
        self.repos = match self.filter {
            Filter::All => self.db.get_all_repositories().await?,
            Filter::GitHub => self.db.get_repositories_by_provider("github").await?,
            Filter::GitLab => self.db.get_repositories_by_provider("gitlab").await?,
            Filter::Cloned => {
                self.db
                    .get_repositories_by_status(CloneStatus::Cloned)
                    .await?
            }
            Filter::NotCloned => {
                self.db
                    .get_repositories_by_status(CloneStatus::NotCloned)
                    .await?
            }
        };

        // Adjust selection if needed
        if self.repos.is_empty() {
            self.selected.select(None);
        } else if let Some(selected) = self.selected.selected() {
            if selected >= self.repos.len() {
                self.selected.select(Some(self.repos.len() - 1));
            }
        } else {
            self.selected.select(Some(0));
        }

        Ok(())
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        // Title
        let title = Paragraph::new("🚀 Super Clone - Repository Manager")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Repository list
        let repos: Vec<ListItem> = self
            .repos
            .iter()
            .map(|repo| {
                let status_icon = match repo.status.as_str() {
                    "cloned" => "✓",
                    "not_cloned" => "○",
                    "cloning" => "⟳",
                    "updating" => "⟳",
                    "error" => "✗",
                    _ => "?",
                };

                let provider_icon = match repo.provider.as_str() {
                    "github" => "🐙",
                    "gitlab" => "🦊",
                    _ => "📦",
                };

                let privacy = if repo.is_private { "🔒" } else { "" };

                let style = match repo.status.as_str() {
                    "cloned" => Style::default().fg(Color::Green),
                    "not_cloned" => Style::default().fg(Color::Gray),
                    "cloning" | "updating" => Style::default().fg(Color::Yellow),
                    "error" => Style::default().fg(Color::Red),
                    _ => Style::default().fg(Color::White),
                };

                let content = format!(
                    "{} {} {} {}",
                    status_icon, provider_icon, privacy, repo.full_name
                );
                ListItem::new(content).style(style)
            })
            .collect();

        let filter_text = match self.filter {
            Filter::All => "All",
            Filter::GitHub => "GitHub",
            Filter::GitLab => "GitLab",
            Filter::Cloned => "Cloned",
            Filter::NotCloned => "Not Cloned",
        };

        let repos_list = List::new(repos)
            .block(Block::default().borders(Borders::ALL).title(format!(
                "Repositories ({}) - {}",
                self.repos.len(),
                filter_text
            )))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(repos_list, chunks[1], &mut self.selected);

        // Status bar
        let status = Paragraph::new(self.status_message.clone())
            .style(Style::default())
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Status"));

        f.render_widget(status, chunks[2]);
    }
}
