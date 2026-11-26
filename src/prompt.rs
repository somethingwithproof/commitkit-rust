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
}

impl Default for TerminalPrompter {
    fn default() -> Self {
        Self::new()
    }
}

impl Prompter for TerminalPrompter {
    fn prompt_prefix(&self, prefixes: &[Prefix]) -> Result<String> {
        let cat = self.locale.catalog();
        let prefix_titles: Vec<String> = prefixes
            .iter()
            .map(|p| format!("{}: {}", p.title, p.description))
            .collect();

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
        let mut display_scopes = vec![cat.prompt_scope_none.to_string()];
        display_scopes.extend(scopes.iter().cloned());

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
        let prompt = fmt1(cat.prompt_subject, "max", &max_length.to_string());

        let subject = Input::with_theme(&self.theme)
            .with_prompt(&prompt)
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err(cat.err_subject_empty)
                } else if input.len() > max_length {
                    Err(cat.err_subject_too_long)
                } else {
                    Ok(())
                }
            })
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
            .context(fmt1(cat.err_prompt_custom, "prompt", prompt))?;

        Ok(input)
    }
}
