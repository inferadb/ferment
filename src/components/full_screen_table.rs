//! Full-screen table view component.
//!
//! A reusable full-screen layout for displaying tabular data with a title bar,
//! optional status badge, and footer with key hints.
//!
//! # Example
//!
//! ```rust,ignore
//! use ferment::components::{FullScreenTable, Column, StatusBadge, BadgeVariant};
//!
//! let view = FullScreenTable::new(80, 24)
//!     .title("My Data View")
//!     .status(StatusBadge::new("Online").variant(BadgeVariant::Success))
//!     .columns(vec![
//!         Column::new("Name").grow(),
//!         Column::new("Value"),
//!     ])
//!     .rows(vec![
//!         vec!["foo".to_string(), "bar".to_string()],
//!     ])
//!     .footer_hints("↑/↓ select  q quit");
//! ```

use crate::components::{Column, StatusBadge, Table};
use crate::runtime::{Cmd, Model};
use crate::style::Color;
use crate::terminal::{Event, KeyCode};
use crate::util::measure_text;

/// Message type for full-screen table.
#[derive(Debug, Clone)]
pub enum FullScreenTableMsg {
    /// Move selection up.
    SelectPrev,
    /// Move selection down.
    SelectNext,
    /// Scroll left (horizontal).
    ScrollLeft,
    /// Scroll right (horizontal).
    ScrollRight,
    /// Page up.
    PageUp,
    /// Page down.
    PageDown,
    /// Quit the view.
    Quit,
    /// Resize the view.
    Resize {
        /// New width in columns.
        width: usize,
        /// New height in rows.
        height: usize,
    },
}

/// Status line content options.
#[derive(Clone)]
pub enum StatusLine {
    /// A status badge (colored text with prefix).
    Badge(StatusBadge),
    /// Custom pre-rendered string (can include ANSI codes).
    Custom(String),
}

impl StatusLine {
    /// Render the status line.
    pub fn render(&self) -> String {
        match self {
            StatusLine::Badge(badge) => badge.render(),
            StatusLine::Custom(s) => s.clone(),
        }
    }
}

/// A full-screen table view with title, status, and footer.
pub struct FullScreenTable {
    /// Terminal width.
    width: usize,
    /// Terminal height.
    height: usize,
    /// Title text.
    title: String,
    /// Optional status line (badge or custom).
    status: Option<StatusLine>,
    /// Table columns.
    columns: Vec<Column>,
    /// Table rows.
    rows: Vec<Vec<String>>,
    /// Footer hints text.
    footer_hints: String,
    /// Current horizontal scroll offset.
    h_scroll_offset: usize,
    /// Selected row index.
    selected_row: usize,
    /// Vertical scroll offset.
    scroll_offset: usize,
    /// Title fill character.
    title_fill_char: char,
    /// Separator character.
    separator_char: char,
}

impl FullScreenTable {
    /// Create a new full-screen table view.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            title: String::new(),
            status: None,
            columns: Vec::new(),
            rows: Vec::new(),
            footer_hints: String::new(),
            h_scroll_offset: 0,
            selected_row: 0,
            scroll_offset: 0,
            title_fill_char: '/',
            separator_char: '─',
        }
    }

    /// Set the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the status badge.
    pub fn status(mut self, status: StatusBadge) -> Self {
        self.status = Some(StatusLine::Badge(status));
        self
    }

    /// Set a custom status line (pre-rendered string with optional ANSI codes).
    pub fn status_custom(mut self, status: impl Into<String>) -> Self {
        self.status = Some(StatusLine::Custom(status.into()));
        self
    }

    /// Set the table columns.
    pub fn columns(mut self, columns: Vec<Column>) -> Self {
        self.columns = columns;
        self
    }

    /// Set the table rows.
    pub fn rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self
    }

    /// Set the footer hints.
    pub fn footer_hints(mut self, hints: impl Into<String>) -> Self {
        self.footer_hints = hints.into();
        self
    }

    /// Set the title fill character (default: '/').
    pub fn title_fill_char(mut self, c: char) -> Self {
        self.title_fill_char = c;
        self
    }

    /// Set the separator character (default: '─').
    pub fn separator_char(mut self, c: char) -> Self {
        self.separator_char = c;
        self
    }

    /// Get the current selected row index.
    pub fn selected_row(&self) -> usize {
        self.selected_row
    }

    /// Get the current row data if any.
    pub fn current_row(&self) -> Option<&Vec<String>> {
        self.rows.get(self.selected_row)
    }

    /// Get the number of visible rows for the table.
    fn visible_rows(&self) -> usize {
        // Layout: title(1) + blank(1) + status(1) + separator(1) + table_header(1) + separator(1) + footer(1)
        self.height.saturating_sub(7)
    }

    /// Ensure scroll is within bounds.
    fn clamp_scroll(&mut self) {
        let row_count = self.rows.len();
        let visible = self.visible_rows();
        let max_scroll = row_count.saturating_sub(visible);
        self.scroll_offset = self.scroll_offset.min(max_scroll);
        self.selected_row = self.selected_row.min(row_count.saturating_sub(1));
    }

    /// Render the title bar.
    fn render_title(&self) -> String {
        let title_prefix = format!("// {} ", self.title);
        let remaining = self.width.saturating_sub(title_prefix.len());
        let fill = self.title_fill_char.to_string().repeat(remaining);
        format!("{}{}", title_prefix, fill)
    }

    /// Render the status line (right-aligned).
    fn render_status_line(&self) -> String {
        if let Some(ref status) = self.status {
            let status_rendered = status.render();
            let status_len = measure_text(&status_rendered);
            let padding = self.width.saturating_sub(status_len);
            format!("{}{}", " ".repeat(padding), status_rendered)
        } else {
            String::new()
        }
    }

    /// Render a separator line.
    fn render_separator(&self) -> String {
        format!(
            "{}{}{}",
            Color::BrightBlack.to_ansi_fg(),
            self.separator_char.to_string().repeat(self.width),
            "\x1b[0m"
        )
    }

    /// Build the table component.
    fn build_table(&self) -> Table {
        Table::new()
            .columns(self.columns.clone())
            .rows(self.rows.clone())
            .height(self.visible_rows())
            .width(self.width)
            .with_h_scroll_offset(self.h_scroll_offset)
            .show_borders(false)
            .header_color(Color::Default)
            .selected_row_color(Color::Cyan)
            .with_cursor_row(self.selected_row)
            .with_offset(self.scroll_offset)
    }

    /// Get the content width of the table.
    fn table_content_width(&self) -> usize {
        self.build_table().content_width()
    }

    /// Maximum horizontal scroll offset.
    fn max_h_scroll(&self) -> usize {
        self.table_content_width().saturating_sub(self.width)
    }

    /// Check if horizontal scrolling is possible.
    fn can_scroll_horizontal(&self) -> bool {
        self.table_content_width() > self.width
    }

    /// Render the footer.
    fn render_footer(&self) -> String {
        // Build left scroll indicator
        let left_indicator = if self.h_scroll_offset > 0 {
            "◀ "
        } else {
            "  "
        };

        // Build right scroll indicator
        let right_indicator =
            if self.can_scroll_horizontal() && self.h_scroll_offset < self.max_h_scroll() {
                " ▶"
            } else {
                "  "
            };

        let indicators_len = 4;
        let hints_len = measure_text(&self.footer_hints);
        let padding = self.width.saturating_sub(hints_len + indicators_len);

        format!(
            "{}{}{}{}{}{}",
            Color::BrightBlack.to_ansi_fg(),
            left_indicator,
            " ".repeat(padding),
            self.footer_hints,
            right_indicator,
            "\x1b[0m"
        )
    }
}

impl Model for FullScreenTable {
    type Message = FullScreenTableMsg;

    fn init(&self) -> Option<Cmd<Self::Message>> {
        None
    }

    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> {
        match msg {
            FullScreenTableMsg::SelectPrev => {
                if self.selected_row > 0 {
                    self.selected_row -= 1;
                    if self.selected_row < self.scroll_offset {
                        self.scroll_offset = self.selected_row;
                    }
                }
            }
            FullScreenTableMsg::SelectNext => {
                if self.selected_row < self.rows.len().saturating_sub(1) {
                    self.selected_row += 1;
                    let visible = self.visible_rows();
                    if self.selected_row >= self.scroll_offset + visible {
                        self.scroll_offset = self.selected_row.saturating_sub(visible - 1);
                    }
                }
            }
            FullScreenTableMsg::ScrollLeft => {
                self.h_scroll_offset = self.h_scroll_offset.saturating_sub(4);
            }
            FullScreenTableMsg::ScrollRight => {
                let max = self.max_h_scroll();
                if self.h_scroll_offset + 4 <= max {
                    self.h_scroll_offset += 4;
                } else {
                    self.h_scroll_offset = max;
                }
            }
            FullScreenTableMsg::PageUp => {
                let page_size = self.visible_rows().saturating_sub(1);
                self.selected_row = self.selected_row.saturating_sub(page_size);
                if self.selected_row < self.scroll_offset {
                    self.scroll_offset = self.selected_row;
                }
            }
            FullScreenTableMsg::PageDown => {
                let page_size = self.visible_rows().saturating_sub(1);
                self.selected_row =
                    (self.selected_row + page_size).min(self.rows.len().saturating_sub(1));
                let visible = self.visible_rows();
                if self.selected_row >= self.scroll_offset + visible {
                    self.scroll_offset = self.selected_row.saturating_sub(visible - 1);
                }
            }
            FullScreenTableMsg::Quit => {
                return Some(Cmd::quit());
            }
            FullScreenTableMsg::Resize { width, height } => {
                self.width = width;
                self.height = height;
            }
        }
        self.clamp_scroll();
        None
    }

    fn view(&self) -> String {
        let mut output = String::new();

        // Title bar
        output.push_str(&self.render_title());
        output.push_str("\r\n");

        // Blank line
        output.push_str("\r\n");

        // Status line (right-aligned)
        output.push_str(&self.render_status_line());
        output.push_str("\r\n");

        // Separator
        output.push_str(&self.render_separator());
        output.push_str("\r\n");

        // Table content
        let table = self.build_table();
        let table_output = table.render();
        let table_lines = table_output.lines().count();

        output.push_str(&table_output.replace('\n', "\r\n"));
        output.push_str("\r\n");

        // Calculate padding needed to push footer to bottom
        let fixed_overhead = 6;
        let total_content_lines = fixed_overhead + table_lines;
        let padding_needed = self.height.saturating_sub(total_content_lines);

        for _ in 0..padding_needed {
            output.push_str("\r\n");
        }

        // Footer separator
        output.push_str(&self.render_separator());
        output.push_str("\r\n");

        // Footer
        output.push_str(&self.render_footer());

        output
    }

    fn handle_event(&self, event: Event) -> Option<Self::Message> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Some(FullScreenTableMsg::Quit),
                KeyCode::Up | KeyCode::Char('k') => Some(FullScreenTableMsg::SelectPrev),
                KeyCode::Down | KeyCode::Char('j') => Some(FullScreenTableMsg::SelectNext),
                KeyCode::Left | KeyCode::Char('h') => Some(FullScreenTableMsg::ScrollLeft),
                KeyCode::Right | KeyCode::Char('l') => Some(FullScreenTableMsg::ScrollRight),
                KeyCode::PageUp => Some(FullScreenTableMsg::PageUp),
                KeyCode::PageDown => Some(FullScreenTableMsg::PageDown),
                _ => None,
            },
            Event::Resize { width, height } => Some(FullScreenTableMsg::Resize {
                width: width as usize,
                height: height as usize,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_screen_table_creation() {
        let view = FullScreenTable::new(80, 24).title("Test View");
        assert_eq!(view.width, 80);
        assert_eq!(view.height, 24);
    }

    #[test]
    fn test_visible_rows() {
        let view = FullScreenTable::new(80, 24);
        // 24 - 7 = 17
        assert_eq!(view.visible_rows(), 17);
    }

    #[test]
    fn test_navigation() {
        let mut view = FullScreenTable::new(80, 24).rows(vec![
            vec!["a".to_string()],
            vec!["b".to_string()],
            vec!["c".to_string()],
        ]);

        assert_eq!(view.selected_row(), 0);
        view.update(FullScreenTableMsg::SelectNext);
        assert_eq!(view.selected_row(), 1);
        view.update(FullScreenTableMsg::SelectPrev);
        assert_eq!(view.selected_row(), 0);
    }
}
