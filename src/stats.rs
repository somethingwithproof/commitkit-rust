// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::process::Command;

/// Statistics for commit types
#[derive(Debug, Default)]
pub struct CommitStats {
    pub total_commits: usize,
    pub type_counts: HashMap<String, usize>,
    pub scope_counts: HashMap<String, usize>,
    pub contributors: HashMap<String, usize>,
    pub commits_by_date: HashMap<String, usize>,
}

pub struct CommitAnalyzer {
    repo_path: String,
}

impl CommitAnalyzer {
    /// Create a new commit analyzer for the given repository
    pub fn new(repo_path: &str) -> Self {
        Self {
            repo_path: repo_path.to_string(),
        }
    }

    /// Analyze commit history and generate statistics
    pub fn analyze_commits(&self, days: Option<u32>) -> Result<CommitStats> {
        let mut stats = CommitStats::default();

        // Build git log command with appropriate format
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.repo_path)
            .arg("log")
            .arg("--pretty=format:%h|%an|%ae|%ad|%s")
            .arg("--date=short");

        // Add date filter if specified
        if let Some(days) = days {
            cmd.arg(format!("--since={} days ago", days));
        }

        let output = cmd.output().context("Failed to run git log")?;
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Compile regex once outside the loop
        let re = regex::Regex::new(r"^(\w+)(?:\(([\w-]+)\))?: .+$").unwrap();

        // Process each commit
        for line in output_str.lines() {
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 5 {
                continue;
            }

            let _hash = parts[0];
            let author = parts[1];
            let date = parts[3];
            let subject = parts[4];

            // Extract commit type and scope using regex
            if let Some(captures) = re.captures(subject) {
                let commit_type = captures.get(1).map_or("", |m| m.as_str()).to_string();

                // Increment type count
                *stats.type_counts.entry(commit_type).or_insert(0) += 1;

                // Extract and count scope if present
                if let Some(scope_match) = captures.get(2) {
                    let scope = scope_match.as_str().to_string();
                    *stats.scope_counts.entry(scope).or_insert(0) += 1;
                }
            }

            // Count by author
            *stats.contributors.entry(author.to_string()).or_insert(0) += 1;

            // Count by date
            *stats.commits_by_date.entry(date.to_string()).or_insert(0) += 1;

            stats.total_commits += 1;
        }

        Ok(stats)
    }

    /// Get commit count by type as a formatted string
    pub fn get_type_summary(&self, days: Option<u32>) -> Result<String> {
        self.get_type_summary_localized(days, crate::i18n::Locale::En)
    }

    /// Get commit count by type as a localized formatted string
    pub fn get_type_summary_localized(
        &self,
        days: Option<u32>,
        locale: crate::i18n::Locale,
    ) -> Result<String> {
        let stats = self.analyze_commits(days)?;
        let cat = locale.catalog();

        let days_str = days.map_or_else(|| cat.stats_days_all.to_string(), |d| d.to_string());
        let mut result = format!(
            "{}\n\n",
            crate::i18n::fmt1(cat.stats_header, "days", &days_str)
        );

        result.push_str(&format!(
            "{}\n\n",
            crate::i18n::fmt1(
                cat.stats_total_commits,
                "count",
                &stats.total_commits.to_string()
            )
        ));

        // Type breakdown
        result.push_str(&format!("{}\n", cat.stats_commit_types));
        let mut type_counts: Vec<(String, usize)> = stats.type_counts.into_iter().collect();
        type_counts.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

        for (commit_type, count) in type_counts {
            let percentage = if stats.total_commits > 0 {
                (count as f64 / stats.total_commits as f64) * 100.0
            } else {
                0.0
            };
            result.push_str(&format!(
                "  {}: {} ({:.1}%)\n",
                commit_type, count, percentage
            ));
        }

        // Top scopes
        if !stats.scope_counts.is_empty() {
            result.push_str(&format!("\n{}\n", cat.stats_top_scopes));
            let mut scope_counts: Vec<(String, usize)> = stats.scope_counts.into_iter().collect();
            scope_counts.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

            for (scope, count) in scope_counts.iter().take(5) {
                result.push_str(&format!("  {}: {}\n", scope, count));
            }
        }

        // Top contributors
        if !stats.contributors.is_empty() {
            result.push_str(&format!("\n{}\n", cat.stats_top_contributors));
            let mut contributor_counts: Vec<(String, usize)> =
                stats.contributors.into_iter().collect();
            contributor_counts.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

            for (contributor, count) in contributor_counts.iter().take(5) {
                let percentage = if stats.total_commits > 0 {
                    (*count as f64 / stats.total_commits as f64) * 100.0
                } else {
                    0.0
                };
                result.push_str(&format!(
                    "  {}: {} ({:.1}%)\n",
                    contributor, count, percentage
                ));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{fmt1, Locale};
    use crate::tests::setup_test_repo;

    fn add_commit(repo_path: &std::path::Path, message: &str) {
        let status = Command::new("git")
            .args(["commit", "--no-gpg-sign", "--allow-empty", "-m", message])
            .current_dir(repo_path)
            .status()
            .expect("Failed to create test commit");
        assert!(status.success(), "git commit failed for message: {message}");
    }

    #[test]
    fn test_analyze_commits_and_english_summary() {
        let repo = setup_test_repo();
        add_commit(repo.path(), "feat(cli): add localized help");
        add_commit(repo.path(), "fix(i18n): handle BCP 47 extensions");
        add_commit(repo.path(), "feat: add second feature without scope");
        add_commit(repo.path(), "non-conventional commit message");

        let analyzer = CommitAnalyzer::new(repo.path().to_str().unwrap());
        let stats = analyzer.analyze_commits(None).unwrap();
        assert_eq!(stats.total_commits, 4);
        assert_eq!(stats.type_counts.get("feat"), Some(&2));
        assert_eq!(stats.type_counts.get("fix"), Some(&1));
        assert_eq!(stats.scope_counts.get("cli"), Some(&1));
        assert_eq!(stats.scope_counts.get("i18n"), Some(&1));
        assert_eq!(stats.contributors.get("Test User"), Some(&4));
        assert!(!stats.commits_by_date.is_empty());

        let summary = analyzer.get_type_summary(None).unwrap();
        let en = Locale::En.catalog();
        assert!(summary.contains(&fmt1(en.stats_header, "days", en.stats_days_all)));
        assert!(summary.contains(&fmt1(en.stats_total_commits, "count", "4")));
        assert!(summary.contains(en.stats_commit_types));
        assert!(summary.contains("feat: 2 (50.0%)"));
        assert!(summary.contains("fix: 1 (25.0%)"));
        assert!(summary.contains(en.stats_top_scopes));
        assert!(summary.contains("cli: 1"));
        assert!(summary.contains("i18n: 1"));
        assert!(summary.contains(en.stats_top_contributors));
        assert!(summary.contains("Test User: 4 (100.0%)"));
    }

    #[test]
    fn test_get_type_summary_localized_multiple_locales_and_days() {
        let repo = setup_test_repo();
        add_commit(repo.path(), "feat(core): add stats localization");
        add_commit(repo.path(), "docs(readme): clarify sign-off trailer");

        let analyzer = CommitAnalyzer::new(repo.path().to_str().unwrap());

        for (locale, days) in [
            (Locale::De, None),
            (Locale::Ja, Some(30)),
            (Locale::Ar, Some(7)),
            (Locale::PtBr, None),
        ] {
            let summary = analyzer.get_type_summary_localized(days, locale).unwrap();
            let cat = locale.catalog();
            let days_str = days.map_or_else(|| cat.stats_days_all.to_string(), |d| d.to_string());

            assert!(summary.contains(&fmt1(cat.stats_header, "days", &days_str)));
            assert!(summary.contains(&fmt1(cat.stats_total_commits, "count", "2")));
            assert!(summary.contains(cat.stats_commit_types));
            assert!(summary.contains(cat.stats_top_scopes));
            assert!(summary.contains(cat.stats_top_contributors));
            assert!(summary.contains("feat: 1 (50.0%)"));
            assert!(summary.contains("docs: 1 (50.0%)"));
            assert!(summary.contains("core: 1"));
            assert!(summary.contains("readme: 1"));
        }
    }

    #[test]
    fn test_get_type_summary_empty_repo() {
        let repo = setup_test_repo();
        let analyzer = CommitAnalyzer::new(repo.path().to_str().unwrap());

        let summary = analyzer
            .get_type_summary_localized(None, Locale::Fr)
            .unwrap();
        let fr = Locale::Fr.catalog();
        assert!(summary.contains(&fmt1(fr.stats_total_commits, "count", "0")));
        assert!(summary.contains(fr.stats_commit_types));
        assert!(!summary.contains(fr.stats_top_scopes));
        assert!(!summary.contains(fr.stats_top_contributors));
    }
}
