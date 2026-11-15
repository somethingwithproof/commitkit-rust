// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Maps commit types to appropriate emojis
pub static COMMIT_TYPE_EMOJIS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("feat", "✨"); // Sparkles for new features
    map.insert("fix", "🐛"); // Bug for bug fixes
    map.insert("docs", "📚"); // Books for documentation
    map.insert("style", "💎"); // Gem for style changes
    map.insert("refactor", "♻️"); // Recycle for refactoring
    map.insert("perf", "🚀"); // Rocket for performance improvements
    map.insert("test", "🧪"); // Test tube for tests
    map.insert("build", "🏗️"); // Construction for build system
    map.insert("ci", "👷"); // Construction worker for CI
    map.insert("chore", "🧹"); // Broom for chores
    map.insert("revert", "⏪"); // Rewind for reverts
    map
});

/// Get emoji for a commit type
pub fn get_emoji_for_type(commit_type: &str) -> Option<&'static str> {
    COMMIT_TYPE_EMOJIS.get(commit_type).copied()
}

/// Apply emoji to commit message if enabled
pub fn apply_emoji(commit_type: &str, message: &str, use_emoji: bool) -> String {
    if !use_emoji {
        return message.to_string();
    }

    if let Some(emoji) = get_emoji_for_type(commit_type) {
        // Find the position after ":" in the commit message
        if let Some(pos) = message.find(':') {
            let (prefix, rest) = message.split_at(pos + 1);
            let trimmed_rest = rest.trim_start();
            return format!("{} {} {}", prefix, emoji, trimmed_rest);
        }
    }

    message.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_insertion() {
        let message = "feat: add new feature";
        let with_emoji = apply_emoji("feat", message, true);
        assert_eq!(with_emoji, "feat: ✨ add new feature");
    }

    #[test]
    fn test_emoji_disabled() {
        let message = "feat: add new feature";
        let with_emoji = apply_emoji("feat", message, false);
        assert_eq!(with_emoji, message);
    }

    #[test]
    fn test_unknown_type() {
        let message = "unknown: some message";
        let with_emoji = apply_emoji("unknown", message, true);
        assert_eq!(with_emoji, message);
    }
}
