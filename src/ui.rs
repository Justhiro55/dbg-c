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
    row_to_match: Vec<Option<usize>>, // Maps table row index to match index (None for separators)
    file_list: Vec<PathBuf>, // List of unique files
    current_file_index: usize, // Index of currently displayed file
}

impl App {
    fn new(matches: Vec<Match>) -> Self {
        let selected = vec![false; matches.len()];

        // Build unique file list
        let mut file_list = Vec::new();
        let mut seen_files = std::collections::HashSet::new();
        for m in &matches {
            if seen_files.insert(m.file_path.clone()) {
                file_list.push(m.file_path.clone());
            }
        }

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
            row_to_match: Vec::new(), // Will be populated in ui()
            file_list,
            current_file_index: 0,
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
            KeyCode::Left | KeyCode::Char('h') => {
                self.previous_file();
                KeyAction::Continue
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.next_file();
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
        let current = self.table_state.selected().unwrap_or(0);
        let max_rows = self.row_to_match.len();

        // Find next selectable row (not a separator)
        let mut next_row = current + 1;
        loop {
            if next_row >= max_rows {
                next_row = 0;
            }
            if self.row_to_match.get(next_row).and_then(|&x| x).is_some() {
                break;
            }
            next_row += 1;
            if next_row == current {
                break; // Avoid infinite loop
            }
        }

        self.table_state.select(Some(next_row));
        self.scroll_state = self.scroll_state.position(next_row);
    }

    fn previous(&mut self) {
        let current = self.table_state.selected().unwrap_or(0);
        let max_rows = self.row_to_match.len();

        // Find previous selectable row (not a separator)
        let mut prev_row = if current == 0 { max_rows - 1 } else { current - 1 };
        loop {
            if self.row_to_match.get(prev_row).and_then(|&x| x).is_some() {
                break;
            }
            if prev_row == 0 {
                prev_row = max_rows - 1;
            } else {
                prev_row -= 1;
            }
            if prev_row == current {
                break; // Avoid infinite loop
            }
        }

        self.table_state.select(Some(prev_row));
        self.scroll_state = self.scroll_state.position(prev_row);
    }

    fn toggle_current(&mut self) {
        if let Some(row_idx) = self.table_state.selected() {
            if let Some(Some(match_idx)) = self.row_to_match.get(row_idx) {
                self.selected[*match_idx] = !self.selected[*match_idx];
                // Move to next item after toggling
                self.next();
            }
        }
    }

    fn toggle_all(&mut self) {
        let all_selected = self.selected.iter().all(|&s| s);
        for s in &mut self.selected {
            *s = !all_selected;
        }
    }

    fn next_file(&mut self) {
        if self.file_list.is_empty() {
            return;
        }
        self.current_file_index = (self.current_file_index + 1) % self.file_list.len();
        // Reset cursor to first row of new file
        self.table_state.select(Some(0));
        self.scroll_state = self.scroll_state.position(0);
    }

    fn previous_file(&mut self) {
        if self.file_list.is_empty() {
            return;
        }
        if self.current_file_index == 0 {
            self.current_file_index = self.file_list.len() - 1;
        } else {
            self.current_file_index -= 1;
        }
        // Reset cursor to first row of new file
        self.table_state.select(Some(0));
        self.scroll_state = self.scroll_state.position(0);
    }

    fn ui(&mut self, f: &mut Frame) {
        let area = f.area();

        // Create layout
        let chunks = Layout::vertical([
            Constraint::Min(3),        // Table
            Constraint::Length(3),     // Help text
        ])
        .split(area);

        // Get current file to filter by
        let current_file = if !self.file_list.is_empty() {
            Some(&self.file_list[self.current_file_index])
        } else {
            None
        };

        // Filter matches to only show current file
        let filtered_matches: Vec<(usize, &Match)> = self
            .matches
            .iter()
            .enumerate()
            .filter(|(_, m)| {
                if let Some(cf) = current_file {
                    &m.file_path == cf
                } else {
                    true
                }
            })
            .collect();

        // Create table rows (no file separators needed when showing single file)
        let mut rows: Vec<Row> = Vec::new();
        let mut row_to_match: Vec<Option<usize>> = Vec::new();

        for (_idx, (original_idx, m)) in filtered_matches.iter().enumerate() {
            let checkbox = if self.selected[*original_idx] { "[✓] " } else { "[ ] " };

            rows.push(Row::new(vec![
                checkbox.to_string(),
                m.line_number.to_string(),
                m.line_content.trim().to_string(),
            ]));
            row_to_match.push(Some(*original_idx)); // Map this row to original match index
        }

        // Store mapping for navigation
        self.row_to_match = row_to_match;

        let selected_count = self.selected.iter().filter(|&&s| s).count();
        let total = self.matches.len();
        let current_pos = self.table_state.selected().unwrap_or(0) + 1;
        let filtered_count = filtered_matches.len();

        let title = if let Some(cf) = current_file {
            format!(
                " {} / {} selected | {} / {} | File {}/{}: {} ",
                selected_count,
                total,
                current_pos,
                filtered_count,
                self.current_file_index + 1,
                self.file_list.len(),
                cf.display()
            )
        } else {
            format!(" {} / {} selected | {} / {} ", selected_count, total, current_pos, total)
        };

        let table = Table::new(
            rows,
            [
                Constraint::Length(4),                  // Checkbox + space
                Constraint::Length(6),                  // Line
                Constraint::Min(20),                    // Code
            ],
        )
        .header(
            Row::new(vec!["   ", "LINE", "CODE"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .bottom_margin(0),
        )
        .block(Block::default().borders(Borders::ALL).title(title))
        .row_highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_spacing(HighlightSpacing::Always)
        .column_spacing(0); // Remove spacing between columns

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
            Span::styled("←/h", Style::default().fg(Color::Cyan)),
            Span::raw(" prev file | "),
            Span::styled("→/l", Style::default().fg(Color::Cyan)),
            Span::raw(" next file | "),
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
