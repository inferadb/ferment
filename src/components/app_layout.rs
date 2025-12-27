//! Full-screen application layout component.
//!
//! Provides a structured layout with title bar, tab bar, content area, and footer.
//!
//! # Example
//!
//! ```rust,ignore
//! use ferment::components::{AppLayout, TabBar, Tab, StatusBadge};
//!
//! let layout = AppLayout::new(80, 24)
//!     .title("My Application")
//!     .tab_bar(
//!         TabBar::new()
//!             .tabs(vec![
//!                 Tab::new("home", "Home").key('h'),
//!                 Tab::new("settings", "Settings").key('s'),
//!             ])
//!     )
//!     .status(StatusBadge::online())
//!     .footer_hints("↑/↓ navigate  q quit");
//! ```

use crate::components::{StatusBadge, Tab, TabBar};
use crate::runtime::{Cmd, Model};
use crate::style::Color;
use crate::terminal::{Event, KeyCode};

/// Message type for app layout.
#[derive(Debug, Clone)]
pub enum AppLayoutMsg {
    /// Switch to a tab by ID.
    SwitchTab(String),
    /// Resize the layout.
    Resize { width: usize, height: usize },
    /// Quit the application.
    Quit,
}

/// A full-screen application layout.
#[derive(Debug, Clone)]
pub struct AppLayout {
    width: usize,
    height: usize,
    title: String,
    tab_bar: Option<TabBar>,
    status: Option<StatusBadge>,
    footer_hints: String,
    content: String,
    title_color: Color,
    separator_color: Color,
    separator_char: char,
    title_fill_char: char,
}

impl AppLayout {
    /// Create a new app layout with dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            title: String::new(),
            tab_bar: None,
            status: None,
            footer_hints: String::new(),
            content: String::new(),
            title_color: Color::BrightBlack,
            separator_color: Color::BrightBlack,
            separator_char: '-',
            title_fill_char: '/',
        }
    }

    /// Set the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the tab bar.
    pub fn tab_bar(mut self, tab_bar: TabBar) -> Self {
        self.tab_bar = Some(tab_bar);
        self
    }

    /// Set the status badge.
    pub fn status(mut self, status: StatusBadge) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the footer hints text.
    pub fn footer_hints(mut self, hints: impl Into<String>) -> Self {
        self.footer_hints = hints.into();
        self
    }

    /// Set the main content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Set the title color.
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Set the separator color.
    pub fn separator_color(mut self, color: Color) -> Self {
        self.separator_color = color;
        self
    }

    /// Set the separator character.
    pub fn separator_char(mut self, c: char) -> Self {
        self.separator_char = c;
        self
    }

    /// Set the title fill character.
    pub fn title_fill_char(mut self, c: char) -> Self {
        self.title_fill_char = c;
        self
    }

    /// Get the current dimensions.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Resize the layout.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    /// Get a mutable reference to the tab bar.
    pub fn tab_bar_mut(&mut self) -> Option<&mut TabBar> {
        self.tab_bar.as_mut()
    }

    /// Get the tab bar.
    pub fn get_tab_bar(&self) -> Option<&TabBar> {
        self.tab_bar.as_ref()
    }

    /// Set the content (mutable).
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    /// Set the status badge (mutable).
    pub fn set_status(&mut self, status: StatusBadge) {
        self.status = Some(status);
    }

    /// Get the number of lines available for content.
    ///
    /// This accounts for: title bar, tab bar, 2 separators, and footer.
    pub fn content_height(&self) -> usize {
        let mut used = 0;
        if !self.title.is_empty() {
            used += 1;
        }
        if self.tab_bar.is_some() {
            used += 1;
        }
        used += 2; // Top separator and footer separator
        if !self.footer_hints.is_empty() {
            used += 1;
        }
        self.height.saturating_sub(used)
    }

    /// Render the title bar.
    fn render_title_bar(&self) -> String {
        if self.title.is_empty() {
            return String::new();
        }

        let title_with_padding = format!("// {} ", self.title);
        let remaining = self.width.saturating_sub(title_with_padding.len());
        let fill = self.title_fill_char.to_string().repeat(remaining);

        format!(
            "{}{}{}{}\n",
            self.title_color.to_ansi_fg(),
            title_with_padding,
            fill,
            "\x1b[0m"
        )
    }

    /// Render the tab bar line.
    fn render_tab_bar_line(&self) -> String {
        let Some(ref tab_bar) = self.tab_bar else {
            return String::new();
        };

        let mut line = tab_bar.render();

        // Add status badge if present
        if let Some(ref status) = self.status {
            let status_str = status.render();
            let status_len = strip_ansi(&status_str).chars().count();
            let tabs_len = strip_ansi(&line).chars().count();
            let padding = self.width.saturating_sub(tabs_len + status_len);

            line.push_str(&" ".repeat(padding));
            line.push_str(&status_str);
        }

        format!("{}\n", line)
    }

    /// Render a separator line.
    fn render_separator(&self) -> String {
        format!(
            "{}{}{}\n",
            self.separator_color.to_ansi_fg(),
            self.separator_char.to_string().repeat(self.width),
            "\x1b[0m"
        )
    }

    /// Render the footer.
    fn render_footer(&self) -> String {
        if self.footer_hints.is_empty() {
            return String::new();
        }

        let hints_len = self.footer_hints.chars().count();
        let padding = self.width.saturating_sub(hints_len);

        format!(
            "{}{}{}{}\n",
            self.separator_color.to_ansi_fg(),
            " ".repeat(padding),
            self.footer_hints,
            "\x1b[0m"
        )
    }

    /// Render the content area.
    fn render_content(&self) -> String {
        let content_height = self.content_height();
        let lines: Vec<&str> = self.content.lines().collect();
        let mut output = String::new();

        for i in 0..content_height {
            if let Some(line) = lines.get(i) {
                // Truncate line if too long
                let truncated: String = line.chars().take(self.width).collect();
                output.push_str(&truncated);
            }
            output.push('\n');
        }

        output
    }

    /// Render the full layout.
    pub fn render(&self) -> String {
        use crate::Model;
        Model::view(self)
    }
}

impl Model for AppLayout {
    type Message = AppLayoutMsg;

    fn init(&self) -> Option<Cmd<Self::Message>> {
        None
    }

    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> {
        match msg {
            AppLayoutMsg::SwitchTab(id) => {
                if let Some(ref mut tab_bar) = self.tab_bar {
                    tab_bar.set_selected(id);
                }
            }
            AppLayoutMsg::Resize { width, height } => {
                self.resize(width, height);
            }
            AppLayoutMsg::Quit => {
                return Some(Cmd::quit());
            }
        }
        None
    }

    fn view(&self) -> String {
        let mut output = String::new();

        // Title bar
        output.push_str(&self.render_title_bar());

        // Tab bar with status
        output.push_str(&self.render_tab_bar_line());

        // Top separator
        output.push_str(&self.render_separator());

        // Content area
        output.push_str(&self.render_content());

        // Footer separator
        output.push_str(&self.render_separator());

        // Footer hints
        output.push_str(&self.render_footer());

        output
    }

    fn handle_event(&self, event: Event) -> Option<Self::Message> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Some(AppLayoutMsg::Quit),
                KeyCode::Char(c) => {
                    // Check if character matches any tab's key
                    if let Some(ref tab_bar) = self.tab_bar {
                        if let Some(id) = tab_bar.tab_for_key(c) {
                            return Some(AppLayoutMsg::SwitchTab(id.to_string()));
                        }
                    }
                    None
                }
                _ => None,
            },
            Event::Resize { width, height } => Some(AppLayoutMsg::Resize {
                width: width as usize,
                height: height as usize,
            }),
            _ => None,
        }
    }
}

/// Strip ANSI escape codes from a string.
fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
            continue;
        }
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
            continue;
        }
        result.push(c);
    }

    result
}

/// Builder for creating tabs quickly.
pub struct TabBuilder {
    tabs: Vec<Tab>,
}

impl TabBuilder {
    /// Create a new tab builder.
    pub fn new() -> Self {
        Self { tabs: Vec::new() }
    }

    /// Add a tab with auto-key.
    pub fn tab(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.tabs.push(Tab::new(id, label).auto_key());
        self
    }

    /// Add a tab with explicit key.
    pub fn tab_with_key(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        key: char,
    ) -> Self {
        self.tabs.push(Tab::new(id, label).key(key));
        self
    }

    /// Build into a TabBar.
    pub fn build(self) -> TabBar {
        TabBar::new().tabs(self.tabs)
    }
}

impl Default for TabBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_creation() {
        let layout = AppLayout::new(80, 24).title("Test App");
        assert_eq!(layout.width, 80);
        assert_eq!(layout.height, 24);
    }

    #[test]
    fn test_content_height() {
        let layout = AppLayout::new(80, 24)
            .title("Test")
            .tab_bar(TabBar::new().tabs(vec![Tab::new("a", "A")]))
            .footer_hints("q quit");

        // title(1) + tab_bar(1) + sep(1) + footer_sep(1) + footer(1) = 5
        // 24 - 5 = 19
        assert_eq!(layout.content_height(), 19);
    }

    #[test]
    fn test_strip_ansi() {
        let input = "\x1b[31mred\x1b[0m text";
        assert_eq!(strip_ansi(input), "red text");
    }

    #[test]
    fn test_tab_builder() {
        let tabs = TabBuilder::new()
            .tab("home", "Home")
            .tab_with_key("settings", "Settings", 's')
            .build();

        assert_eq!(tabs.get_tabs().len(), 2);
        assert_eq!(tabs.get_tabs()[0].key, Some('h'));
        assert_eq!(tabs.get_tabs()[1].key, Some('s'));
    }
}
