//! Status badge component for displaying status indicators.
//!
//! A colored status indicator with optional icon/emoji and label.
//!
//! # Example
//!
//! ```rust
//! use ferment::components::{StatusBadge, BadgeVariant};
//!
//! // Using preset variants
//! let online = StatusBadge::online();
//! let offline = StatusBadge::offline();
//!
//! // Custom badge
//! let custom = StatusBadge::new("Building")
//!     .icon("🔨")
//!     .variant(BadgeVariant::Info);
//! ```

use crate::runtime::{Cmd, Model};
use crate::style::Color;
use crate::terminal::Event;

/// Preset variants for status badges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    /// Success/Online status (green).
    Success,
    /// Error/Offline status (red).
    Error,
    /// Warning/Paused status (yellow).
    Warning,
    /// Info status (blue).
    Info,
    /// Neutral/Unknown status (gray).
    #[default]
    Neutral,
}

impl BadgeVariant {
    /// Get the default icon for this variant.
    pub fn icon(&self) -> &'static str {
        match self {
            BadgeVariant::Success => "●",
            BadgeVariant::Error => "●",
            BadgeVariant::Warning => "●",
            BadgeVariant::Info => "●",
            BadgeVariant::Neutral => "○",
        }
    }

    /// Get the default emoji icon for this variant.
    pub fn emoji(&self) -> &'static str {
        match self {
            BadgeVariant::Success => "🟢",
            BadgeVariant::Error => "🔴",
            BadgeVariant::Warning => "🟡",
            BadgeVariant::Info => "🔵",
            BadgeVariant::Neutral => "⚪",
        }
    }

    /// Get the color for this variant.
    pub fn color(&self) -> Color {
        match self {
            BadgeVariant::Success => Color::Green,
            BadgeVariant::Error => Color::Red,
            BadgeVariant::Warning => Color::Yellow,
            BadgeVariant::Info => Color::Blue,
            BadgeVariant::Neutral => Color::BrightBlack,
        }
    }
}

/// Message type for status badge (currently none needed).
#[derive(Debug, Clone)]
pub enum StatusBadgeMsg {}

/// A colored status indicator badge.
#[derive(Debug, Clone)]
pub struct StatusBadge {
    label: String,
    icon: Option<String>,
    variant: BadgeVariant,
    color: Option<Color>,
    use_emoji: bool,
    show_icon: bool,
}

impl Default for StatusBadge {
    fn default() -> Self {
        Self {
            label: String::new(),
            icon: None,
            variant: BadgeVariant::Neutral,
            color: None,
            use_emoji: true,
            show_icon: true,
        }
    }
}

impl StatusBadge {
    /// Create a new status badge with a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    /// Create an "Online" badge.
    pub fn online() -> Self {
        Self::new("Online").variant(BadgeVariant::Success)
    }

    /// Create an "Offline" badge.
    pub fn offline() -> Self {
        Self::new("Offline").variant(BadgeVariant::Error)
    }

    /// Create a "Paused" badge.
    pub fn paused() -> Self {
        Self::new("Paused").variant(BadgeVariant::Warning)
    }

    /// Create a "Loading" badge.
    pub fn loading() -> Self {
        Self::new("Loading").variant(BadgeVariant::Info)
    }

    /// Create an "Unknown" badge.
    pub fn unknown() -> Self {
        Self::new("Unknown").variant(BadgeVariant::Neutral)
    }

    /// Set the badge variant.
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set a custom icon (overrides variant icon).
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set a custom color (overrides variant color).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Use emoji icons (default: true).
    pub fn use_emoji(mut self, use_emoji: bool) -> Self {
        self.use_emoji = use_emoji;
        self
    }

    /// Show the icon (default: true).
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Set the label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Get the effective icon.
    fn effective_icon(&self) -> &str {
        if let Some(ref icon) = self.icon {
            icon
        } else if self.use_emoji {
            self.variant.emoji()
        } else {
            self.variant.icon()
        }
    }

    /// Get the effective color.
    fn effective_color(&self) -> Color {
        self.color.clone().unwrap_or_else(|| self.variant.color())
    }

    /// Render the badge as a string.
    pub fn render(&self) -> String {
        use crate::Model;
        Model::view(self)
    }
}

impl Model for StatusBadge {
    type Message = StatusBadgeMsg;

    fn init(&self) -> Option<Cmd<Self::Message>> {
        None
    }

    fn update(&mut self, _msg: Self::Message) -> Option<Cmd<Self::Message>> {
        None
    }

    fn view(&self) -> String {
        let mut output = String::new();

        if self.show_icon {
            let icon = self.effective_icon();
            // For emoji icons, we don't need color codes
            if self.use_emoji && self.icon.is_none() {
                output.push_str(icon);
            } else {
                output.push_str(&format!(
                    "{}{}{}",
                    self.effective_color().to_ansi_fg(),
                    icon,
                    "\x1b[0m"
                ));
            }
            output.push(' ');
        }

        output.push_str(&self.label);
        output
    }

    fn handle_event(&self, _event: Event) -> Option<Self::Message> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_creation() {
        let badge = StatusBadge::new("Test");
        assert_eq!(badge.label, "Test");
        assert_eq!(badge.variant, BadgeVariant::Neutral);
    }

    #[test]
    fn test_preset_badges() {
        let online = StatusBadge::online();
        assert_eq!(online.label, "Online");
        assert_eq!(online.variant, BadgeVariant::Success);

        let offline = StatusBadge::offline();
        assert_eq!(offline.label, "Offline");
        assert_eq!(offline.variant, BadgeVariant::Error);
    }

    #[test]
    fn test_variant_colors() {
        assert!(matches!(BadgeVariant::Success.color(), Color::Green));
        assert!(matches!(BadgeVariant::Error.color(), Color::Red));
        assert!(matches!(BadgeVariant::Warning.color(), Color::Yellow));
    }

    #[test]
    fn test_variant_emojis() {
        assert_eq!(BadgeVariant::Success.emoji(), "🟢");
        assert_eq!(BadgeVariant::Error.emoji(), "🔴");
        assert_eq!(BadgeVariant::Warning.emoji(), "🟡");
    }

    #[test]
    fn test_custom_icon() {
        let badge = StatusBadge::new("Building").icon("🔨");
        assert_eq!(badge.effective_icon(), "🔨");
    }

    #[test]
    fn test_render_contains_label() {
        let badge = StatusBadge::online();
        let rendered = badge.render();
        assert!(rendered.contains("Online"));
    }
}
