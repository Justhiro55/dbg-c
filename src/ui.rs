use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Table, TableState,
    },
    Frame, Terminal,
};
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

use crate::types::Match;

pub fn display_matches(matches: &[Match]) {
    // Display all matches grouped by file
    println!("\nFound {} debug statement(s):\n", matches.len());

    // Group matches by file
    let mut files_map: HashMap<PathBuf, Vec<&Match>> = HashMap::new();

    for m in matches {
        files_map.entry(m.file_path.clone()).or_default().push(m);
    }

    // Sort files by path for consistent display
    let mut sorted_files: Vec<_> = files_map.iter().collect();
    sorted_files.sort_by_key(|(path, _)| path.as_path());

    for (file_path, file_matches) in sorted_files {
        // Display filename in color (magenta like ripgrep)
        println!("\x1b[35m{}\x1b[0m", file_path.display());

        // Sort matches by line number
        let mut sorted_matches = file_matches.clone();
        sorted_matches.sort_by_key(|m| m.line_number);

        for m in sorted_matches {
            // Line number in green, followed by colon and content with highlighted debug keyword
            let highlighted = highlight_debug_keyword(&m.line_content);
            println!("\x1b[32m{}\x1b[0m:{}", m.line_number, highlighted.trim());
        }

        println!(); // Empty line between files
    }
}

pub fn select_statements_interactive(matches: &[Match]) -> Result<Vec<Match>> {
    if matches.is_empty() {
        return Ok(vec![]);
    }

    let mut app = App::new(matches.to_vec());
    let selected = app.run()?;

    Ok(selected)
}

fn highlight_debug_keyword(line: &str) -> String {
    // Highlight "debug" or "DEBUG" keywords in red
    let re = Regex::new(r"(debug|DEBUG)").unwrap();
    re.replace_all(line, "\x1b[1;31m$1\x1b[0m").to_string()
}

struct App {
    matches: Vec<Match>,
    table_state: TableState,
    scroll_state: ScrollbarState,
    selected: Vec<bool>, // Track which items are selected
}

impl App {
    fn new(matches: Vec<Match>) -> Self {
        let selected = vec![false; matches.len()];
        let scroll_state = ScrollbarState::new(matches.len());
        let mut table_state = TableState::default();
        if !matches.is_empty() {
            table_state.select(Some(0));
        }

        Self {
            matches,
            table_state,
            scroll_state,
            selected,
        }
    }

    fn run(&mut self) -> Result<Vec<Match>> {
        // Setup terminal with inline viewport (keeps CLI history)
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);

        // Use inline mode to preserve terminal history
        let height = (self.matches.len() as u16 + 6).min(30); // +6 for borders and headers
        let mut terminal = Terminal::with_options(
            backend,
            ratatui::TerminalOptions {
                viewport: ratatui::Viewport::Inline(height),
            },
        )?;

        let result = self.run_app(&mut terminal);

        // Restore terminal (no need for LeaveAlternateScreen in inline mode)
        disable_raw_mode()?;
        terminal.show_cursor()?;

        result
    }

    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<Vec<Match>> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.handle_key(key) {
                        KeyAction::Quit => return Ok(vec![]),
                        KeyAction::Confirm => {
                            let selected: Vec<Match> = self
                                .matches
                                .iter()
                                .enumerate()
                                .filter(|(i, _)| self.selected[*i])
                                .map(|(_, m)| m.clone())
                                .collect();
                            return Ok(selected);
                        }
                        KeyAction::Continue => {}
                    }
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> KeyAction {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => KeyAction::Quit,
            KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                KeyAction::Quit
            }
            KeyCode::Enter => KeyAction::Confirm,
            KeyCode::Down | KeyCode::Char('j') => {
                self.next();
                KeyAction::Continue
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous();
                KeyAction::Continue
            }
            KeyCode::Tab | KeyCode::Char(' ') => {
                self.toggle_current();
                KeyAction::Continue
            }
            KeyCode::Char('a') => {
                self.toggle_all();
                KeyAction::Continue
            }
            _ => KeyAction::Continue,
        }
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.matches.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.matches.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    fn toggle_current(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.selected[i] = !self.selected[i];
        }
    }

    fn toggle_all(&mut self) {
        let all_selected = self.selected.iter().all(|&s| s);
        for s in &mut self.selected {
            *s = !all_selected;
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let area = f.area();

        // Create layout
        let chunks = Layout::vertical([
            Constraint::Min(3),        // Table
            Constraint::Length(3),     // Help text
        ])
        .split(area);

        // Calculate column widths
        let max_file_width = self
            .matches
            .iter()
            .map(|m| m.file_path.display().to_string().len())
            .max()
            .unwrap_or(20)
            .min(40);

        // Create table rows
        let rows: Vec<Row> = self
            .matches
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let checkbox = if self.selected[i] { "[✓]" } else { "[ ]" };
                let file_str = m.file_path.display().to_string();
                let file_display = if file_str.len() > max_file_width {
                    format!("...{}", &file_str[file_str.len() - max_file_width + 3..])
                } else {
                    file_str
                };

                Row::new(vec![
                    checkbox.to_string(),
                    file_display,
                    m.line_number.to_string(),
                    m.line_content.trim().to_string(),
                ])
            })
            .collect();

        let selected_count = self.selected.iter().filter(|&&s| s).count();
        let total = self.matches.len();
        let current_pos = self.table_state.selected().unwrap_or(0) + 1;

        let title = format!(" {} / {} selected | {} / {} ", selected_count, total, current_pos, total);

        let table = Table::new(
            rows,
            [
                Constraint::Length(3),                  // Checkbox
                Constraint::Length(max_file_width as u16 + 2), // File
                Constraint::Length(6),                  // Line
                Constraint::Min(20),                    // Code
            ],
        )
        .header(
            Row::new(vec!["   ", "FILE", "LINE", "CODE"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .bottom_margin(0),
        )
        .block(Block::default().borders(Borders::ALL).title(title))
        .row_highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_spacing(HighlightSpacing::Always);

        f.render_stateful_widget(table, chunks[0], &mut self.table_state);

        // Render scrollbar
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = self.scroll_state.clone();
        f.render_stateful_widget(
            scrollbar,
            chunks[0].inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );

        // Help text
        let help = Line::from(vec![
            Span::styled("↑/k", Style::default().fg(Color::Cyan)),
            Span::raw(" up | "),
            Span::styled("↓/j", Style::default().fg(Color::Cyan)),
            Span::raw(" down | "),
            Span::styled("Space/Tab", Style::default().fg(Color::Cyan)),
            Span::raw(" toggle | "),
            Span::styled("a", Style::default().fg(Color::Cyan)),
            Span::raw(" all | "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(" confirm | "),
            Span::styled("Esc/q/Ctrl-C", Style::default().fg(Color::Red)),
            Span::raw(" cancel"),
        ]);

        f.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .title(" Controls "),
            chunks[1],
        );

        f.render_widget(
            ratatui::widgets::Paragraph::new(help).block(Block::default()),
            chunks[1].inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 1,
            }),
        );
    }
}

enum KeyAction {
    Continue,
    Quit,
    Confirm,
}
