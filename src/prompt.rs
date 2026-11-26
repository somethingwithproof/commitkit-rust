// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::config::Prefix;
use crate::i18n::{fmt1, Locale};

/// Trait defining the interaction methods for commit message creation
pub trait Prompter {
    fn prompt_prefix(&self, prefixes: &[Prefix]) -> Result<String>;
    fn prompt_scope(&self, scopes: &[String]) -> Result<String>;
    fn prompt_subject(&self, max_length: usize) -> Result<String>;
    fn prompt_body(&self) -> Result<String>;
    fn prompt_footer(&self) -> Result<String>;
    fn prompt_custom(&self, prompt: &str) -> Result<String>;
}

/// Terminal-based implementation of the Prompter trait using dialoguer
pub struct TerminalPrompter {
    theme: ColorfulTheme,
    locale: Locale,
}

impl TerminalPrompter {
    pub fn new() -> Self {
        Self::with_locale(Locale::detect())
    }

    pub fn with_locale(locale: Locale) -> Self {
        Self {
            theme: ColorfulTheme::default(),
            locale,
        }
    }

    fn format_prefix_items(prefixes: &[Prefix]) -> Vec<String> {
        prefixes
            .iter()
            .map(|p| format!("{}: {}", p.title, p.description))
            .collect()
    }

    fn scope_items(&self, scopes: &[String]) -> Vec<String> {
        let cat = self.locale.catalog();
        let mut display_scopes = vec![cat.prompt_scope_none.to_string()];
        display_scopes.extend(scopes.iter().cloned());
        display_scopes
    }

    fn subject_prompt(&self, max_length: usize) -> String {
        let cat = self.locale.catalog();
        fmt1(cat.prompt_subject, "max", &max_length.to_string())
    }

    fn validate_subject(&self, input: &str, max_length: usize) -> Result<(), &'static str> {
        let cat = self.locale.catalog();
        if input.trim().is_empty() {
            Err(cat.err_subject_empty)
        } else if input.len() > max_length {
            Err(cat.err_subject_too_long)
        } else {
            Ok(())
        }
    }
}

impl Default for TerminalPrompter {
    fn default() -> Self {
        Self::new()
    }
}

impl Prompter for TerminalPrompter {
    fn prompt_prefix(&self, prefixes: &[Prefix]) -> Result<String> {
        let cat = self.locale.catalog();
        let prefix_titles = Self::format_prefix_items(prefixes);

        let selection = Select::with_theme(&self.theme)
            .with_prompt(cat.prompt_select_type)
            .items(&prefix_titles)
            .default(0)
            .interact()
            .context(cat.err_prompt_prefix)?;

        Ok(prefixes[selection].title.clone())
    }

    fn prompt_scope(&self, scopes: &[String]) -> Result<String> {
        if scopes.is_empty() {
            return Ok(String::new());
        }

        let cat = self.locale.catalog();
        // Add "None" as the first option
        let display_scopes = self.scope_items(scopes);

        let selection = Select::with_theme(&self.theme)
            .with_prompt(cat.prompt_select_scope)
            .items(&display_scopes)
            .default(0)
            .interact()
            .context(cat.err_prompt_scope)?;

        // Return empty string if "None" was selected
        if selection == 0 {
            Ok(String::new())
        } else {
            Ok(scopes[selection - 1].clone())
        }
    }

    fn prompt_subject(&self, max_length: usize) -> Result<String> {
        let cat = self.locale.catalog();
        let prompt = self.subject_prompt(max_length);

        let subject = Input::with_theme(&self.theme)
            .with_prompt(&prompt)
            .validate_with(|input: &String| self.validate_subject(input, max_length))
            .interact_text()
            .context(cat.err_prompt_subject)?;

        Ok(subject)
    }

    fn prompt_body(&self) -> Result<String> {
        let cat = self.locale.catalog();
        println!("{}", style(cat.prompt_body).bold());
        println!("{}", style(cat.prompt_body_hint).dim());

        let mut body = String::new();
        let stdin = std::io::stdin();

        while let Ok(n) = stdin.read_line(&mut body) {
            if n == 0 || body.trim().is_empty() {
                break;
            }
        }

        Ok(body.trim().to_string())
    }

    fn prompt_footer(&self) -> Result<String> {
        let cat = self.locale.catalog();
        let footer = Input::<String>::with_theme(&self.theme)
            .with_prompt(cat.prompt_footer)
            .allow_empty(true)
            .interact_text()
            .context(cat.err_prompt_footer)?;

        Ok(footer)
    }

    fn prompt_custom(&self, prompt: &str) -> Result<String> {
        let cat = self.locale.catalog();
        let input = Input::<String>::with_theme(&self.theme)
            .with_prompt(prompt)
            .interact_text()
            .with_context(|| fmt1(cat.err_prompt_custom, "prompt", prompt))?;

        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockPrompter {
        prefix: Result<String, String>,
        scope: Result<String, String>,
        subject: Result<String, String>,
        body: Result<String, String>,
        footer: Result<String, String>,
        custom: Result<String, String>,
    }

    impl MockPrompter {
        fn ok() -> Self {
            Self {
                prefix: Ok("feat".to_string()),
                scope: Ok("i18n".to_string()),
                subject: Ok("add localization support".to_string()),
                body: Ok("Detailed body".to_string()),
                footer: Ok("Closes #42".to_string()),
                custom: Ok("custom value".to_string()),
            }
        }
    }

    impl Prompter for MockPrompter {
        fn prompt_prefix(&self, _prefixes: &[Prefix]) -> Result<String> {
            self.prefix.clone().map_err(|e| anyhow!(e))
        }

        fn prompt_scope(&self, _scopes: &[String]) -> Result<String> {
            self.scope.clone().map_err(|e| anyhow!(e))
        }

        fn prompt_subject(&self, _max_length: usize) -> Result<String> {
            self.subject.clone().map_err(|e| anyhow!(e))
        }

        fn prompt_body(&self) -> Result<String> {
            self.body.clone().map_err(|e| anyhow!(e))
        }

        fn prompt_footer(&self) -> Result<String> {
            self.footer.clone().map_err(|e| anyhow!(e))
        }

        fn prompt_custom(&self, _prompt: &str) -> Result<String> {
            self.custom.clone().map_err(|e| anyhow!(e))
        }
    }

    #[test]
    fn test_terminal_prompter_constructors_and_empty_scope() {
        let _default_prompter = TerminalPrompter::default();
        let _new_prompter = TerminalPrompter::new();
        let prompter = TerminalPrompter::with_locale(Locale::De);
        assert_eq!(prompter.locale, Locale::De);
        assert_eq!(prompter.prompt_scope(&[]).unwrap(), "");
    }

    #[test]
    fn test_localized_prompt_helpers() {
        let prefixes = vec![
            Prefix {
                title: "feat".to_string(),
                description: "A new feature".to_string(),
            },
            Prefix {
                title: "fix".to_string(),
                description: "A bug fix".to_string(),
            },
        ];
        assert_eq!(
            TerminalPrompter::format_prefix_items(&prefixes),
            vec!["feat: A new feature", "fix: A bug fix"]
        );

        let scopes = vec!["core".to_string(), "cli".to_string()];
        let en = TerminalPrompter::with_locale(Locale::En);
        assert_eq!(en.scope_items(&scopes), vec!["None", "core", "cli"]);
        assert!(en.subject_prompt(72).contains("72"));

        let de = TerminalPrompter::with_locale(Locale::De);
        assert_eq!(
            de.scope_items(&scopes),
            vec![Locale::De.catalog().prompt_scope_none, "core", "cli"]
        );
        assert!(de.subject_prompt(50).contains("50"));

        let ja = TerminalPrompter::with_locale(Locale::Ja);
        assert_eq!(
            ja.scope_items(&scopes)[0],
            Locale::Ja.catalog().prompt_scope_none
        );
        assert!(ja.subject_prompt(60).contains("60"));
    }

    #[test]
    fn test_validate_subject_localized() {
        for locale in [Locale::En, Locale::De, Locale::Ja, Locale::Ar] {
            let prompter = TerminalPrompter::with_locale(locale);
            let cat = locale.catalog();

            assert_eq!(
                prompter.validate_subject("", 20),
                Err(cat.err_subject_empty)
            );
            assert_eq!(
                prompter.validate_subject("   \t  ", 20),
                Err(cat.err_subject_empty)
            );
            assert_eq!(
                prompter.validate_subject("this subject exceeds twenty chars", 20),
                Err(cat.err_subject_too_long)
            );
            assert_eq!(
                prompter.validate_subject("12345678901234567890", 20),
                Ok(())
            );
            assert_eq!(prompter.validate_subject("valid subject", 20), Ok(()));
        }
    }

    #[test]
    fn test_mock_prompter_success_and_error_paths() {
        let prompter = MockPrompter::ok();
        let prefixes = vec![Prefix {
            title: "feat".to_string(),
            description: "New feature".to_string(),
        }];
        let scopes = vec!["i18n".to_string()];

        assert_eq!(prompter.prompt_prefix(&prefixes).unwrap(), "feat");
        assert_eq!(prompter.prompt_scope(&scopes).unwrap(), "i18n");
        assert_eq!(
            prompter.prompt_subject(72).unwrap(),
            "add localization support"
        );
        assert_eq!(prompter.prompt_body().unwrap(), "Detailed body");
        assert_eq!(prompter.prompt_footer().unwrap(), "Closes #42");
        assert_eq!(prompter.prompt_custom("ticket").unwrap(), "custom value");

        let failing = MockPrompter {
            prefix: Err(Locale::De.catalog().err_prompt_prefix.to_string()),
            scope: Err(Locale::De.catalog().err_prompt_scope.to_string()),
            subject: Err(Locale::De.catalog().err_prompt_subject.to_string()),
            body: Err("body error".to_string()),
            footer: Err(Locale::De.catalog().err_prompt_footer.to_string()),
            custom: Err(fmt1(
                Locale::De.catalog().err_prompt_custom,
                "prompt",
                "ticket",
            )),
        };
        assert_eq!(
            failing.prompt_prefix(&prefixes).unwrap_err().to_string(),
            Locale::De.catalog().err_prompt_prefix
        );
        assert_eq!(
            failing.prompt_scope(&scopes).unwrap_err().to_string(),
            Locale::De.catalog().err_prompt_scope
        );
        assert_eq!(
            failing.prompt_subject(72).unwrap_err().to_string(),
            Locale::De.catalog().err_prompt_subject
        );
        assert!(failing.prompt_body().is_err());
        assert_eq!(
            failing.prompt_footer().unwrap_err().to_string(),
            Locale::De.catalog().err_prompt_footer
        );
        assert!(failing
            .prompt_custom("ticket")
            .unwrap_err()
            .to_string()
            .contains("ticket"));
    }
}
