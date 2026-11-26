// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use clap::{CommandFactory, FromArgMatches, Parser};
use std::process;

use commitkit::config::Config;
use commitkit::git;
use commitkit::i18n::{extract_cli_lang, fmt1, fmt2, Locale};
use commitkit::prompt::{Prompter, TerminalPrompter};

/// CLI tool for creating conventional commits
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Skip git commit and print the message to stdout
    #[clap(short, long)]
    dry_run: bool,

    /// Show verbose output
    #[clap(short, long)]
    verbose: bool,

    /// Path to config file (default: .commitkit.toml in current or home directory)
    #[clap(short, long)]
    config: Option<String>,

    /// Toggle emoji in commit messages
    #[clap(long)]
    emoji: bool,

    /// Use a specific commit template
    #[clap(long)]
    template: Option<String>,

    /// Update changelog after committing
    #[clap(long)]
    changelog: bool,

    /// Install git hooks
    #[clap(long)]
    install_hooks: bool,

    /// Show commit statistics
    #[clap(long)]
    stats: bool,

    /// Number of days to analyze for stats (default: all)
    #[clap(long)]
    days: Option<u32>,

    /// Validate a commit message file
    #[clap(long)]
    validate: Option<String>,

    /// Prepare commit message (used by git hook)
    #[clap(long)]
    prepare_msg: Option<String>,

    /// Display language (e.g. en, de, ja, zh-CN, pt-BR)
    #[clap(long)]
    lang: Option<String>,
}

fn localized_command(locale: Locale) -> clap::Command {
    let cat = locale.catalog();
    Args::command()
        .about(cat.cli_about)
        .mut_arg("dry_run", |a| a.help(cat.cli_opt_dry_run))
        .mut_arg("verbose", |a| a.help(cat.cli_opt_verbose))
        .mut_arg("config", |a| a.help(cat.cli_opt_config))
        .mut_arg("emoji", |a| a.help(cat.cli_opt_emoji))
        .mut_arg("template", |a| a.help(cat.cli_opt_template))
        .mut_arg("changelog", |a| a.help(cat.cli_opt_changelog))
        .mut_arg("install_hooks", |a| a.help(cat.cli_opt_install_hooks))
        .mut_arg("stats", |a| a.help(cat.cli_opt_stats))
        .mut_arg("days", |a| a.help(cat.cli_opt_days))
        .mut_arg("validate", |a| a.help(cat.cli_opt_validate))
        .mut_arg("prepare_msg", |a| a.help(cat.cli_opt_prepare_msg))
        .mut_arg("lang", |a| a.help(cat.cli_opt_lang))
}

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();
    let initial_locale = Locale::resolve(extract_cli_lang(&raw_args).as_deref());

    if let Err(err) = run(&raw_args, initial_locale) {
        let cat = initial_locale.catalog();
        eprintln!("{}", fmt1(cat.err_prefix, "error", &err.to_string()));
        process::exit(1);
    }
}

fn run(raw_args: &[String], initial_locale: Locale) -> Result<()> {
    let matches = localized_command(initial_locale).get_matches_from(raw_args);
    let args = Args::from_arg_matches(&matches)?;
    let locale = Locale::resolve(args.lang.as_deref());
    let cat = locale.catalog();

    // Load config (custom path if provided)
    let config = if let Some(config_path) = &args.config {
        let path = std::path::PathBuf::from(config_path);
        if !path.exists() {
            return Err(anyhow::anyhow!(fmt1(
                cat.err_config_not_found,
                "path",
                config_path
            )));
        }

        let config_str = std::fs::read_to_string(&path).context(fmt1(
            cat.err_config_read,
            "path",
            config_path,
        ))?;

        toml::from_str(&config_str).context(fmt1(cat.err_config_parse, "path", config_path))?
    } else {
        Config::load()?
    };

    // Handle special commands that don't require a git repo
    if args.install_hooks {
        return install_git_hooks(&args, locale);
    }

    if let Some(validate_file) = &args.validate {
        return validate_commit_message(validate_file, &config, locale);
    }

    if let Some(prepare_msg) = &args.prepare_msg {
        return prepare_commit_message(prepare_msg, &config);
    }

    if args.stats {
        return show_commit_stats(&args, locale);
    }

    // The main commit workflow requires a git repo
    if !git::is_git_repo()? {
        return Err(anyhow::anyhow!(cat.err_not_git_repo));
    }

    // Check if there are staged changes to commit
    if !args.dry_run && !git::has_staged_changes()? {
        return Err(anyhow::anyhow!(cat.err_no_staged_changes));
    }

    if args.verbose {
        println!("{}", cat.msg_loading_config);
    }

    // Determine if emoji should be used (CLI argument overrides config)
    let use_emoji = if args.emoji { true } else { config.use_emoji };

    // Determine if changelog should be updated (CLI argument overrides config)
    let update_changelog = if args.changelog {
        true
    } else {
        config.update_changelog
    };

    if args.verbose {
        println!("{}", cat.msg_starting_interactive);
    }

    // Handle template-based commit if requested
    if let Some(template_name) = &args.template {
        return create_template_commit(
            template_name,
            &config,
            use_emoji,
            update_changelog,
            &args,
            locale,
        );
    }

    // Standard interactive commit flow
    let prompter = TerminalPrompter::with_locale(locale);

    let prefix = prompter.prompt_prefix(&config.prefixes)?;
    let scope = prompter.prompt_scope(&config.scopes)?;
    let subject = prompter.prompt_subject(config.max_subject_len)?;
    let body = prompter.prompt_body()?;
    let footer = prompter.prompt_footer()?;

    // Build commit message with or without emoji
    let commit_message = if use_emoji {
        commitkit::build_commit_message_with_emoji(&prefix, &scope, &subject, &body, &footer, true)
    } else {
        commitkit::build_commit_message(&prefix, &scope, &subject, &body, &footer)
    };

    if args.verbose {
        println!("{}", cat.msg_generated);
    }

    // Handle dry run
    if args.dry_run {
        println!("{}", cat.dry_run_header);
        println!("{}", commit_message);
        println!("---------------------");
        if args.verbose {
            println!("{}", cat.msg_dry_run_notice);
        }
        return Ok(());
    }

    // Execute the commit
    if args.verbose {
        println!("{}", cat.msg_executing_commit);
    }

    git::run_git_commit(&commit_message, config.sign_off_commits).context(cat.err_commit_failed)?;

    println!("{}", cat.msg_commit_success);

    // Update changelog if enabled
    if update_changelog {
        if args.verbose {
            println!("{}", cat.msg_updating_changelog);
        }

        let current_dir = std::env::current_dir()?;
        let changelog_path = current_dir.join("CHANGELOG.md");

        let changelog = commitkit::changelog::ChangelogManager::new(
            &changelog_path,
            &current_dir
                .file_name()
                .unwrap_or_default()
                .to_string_lossy(),
        );

        changelog
            .add_entry(&prefix, Some(&scope), &subject, Some(&body))
            .context(cat.err_changelog_update)?;

        if args.verbose {
            println!("{}", cat.msg_changelog_updated);
        }
    }

    Ok(())
}

/// Install git hooks
fn install_git_hooks(args: &Args, locale: Locale) -> Result<()> {
    let cat = locale.catalog();

    // Find the git repo
    let current_dir = std::env::current_dir()?;
    let repo_root = commitkit::hooks::GitHookManager::find_repo_root(&current_dir)
        .ok_or_else(|| anyhow::anyhow!(cat.err_not_git_repo))?;

    let hook_manager = commitkit::hooks::GitHookManager::new(&repo_root);

    // Install the hooks
    hook_manager
        .install_prepare_commit_msg_hook()
        .context(cat.err_install_prepare_hook)?;

    hook_manager
        .install_commit_msg_hook()
        .context(cat.err_install_commit_msg_hook)?;

    println!("{}", cat.msg_hooks_installed);

    if args.verbose {
        println!("{}", cat.msg_installed_hooks_header);
        println!("  - {}", cat.msg_hook_prepare_desc);
        println!("  - {}", cat.msg_hook_validate_desc);
    }

    Ok(())
}

/// Validate a commit message against conventional commits standard
fn validate_commit_message(file_path: &str, config: &Config, locale: Locale) -> Result<()> {
    let cat = locale.catalog();
    let validator = commitkit::hooks::CommitMessageValidator::new()
        .min_subject_length(config.min_subject_len)
        .max_subject_length(config.max_subject_len)
        .required_types(config.prefixes.iter().map(|p| p.title.clone()).collect());

    match validator.validate_file(file_path) {
        Ok(_) => {
            println!("{}", cat.msg_commit_valid);
            Ok(())
        }
        Err(err) => {
            let error_message = match err {
                commitkit::hooks::ValidationError::InvalidFormat => cat.err_validation_format,
                commitkit::hooks::ValidationError::InvalidType => cat.err_validation_type,
                commitkit::hooks::ValidationError::SubjectTooShort => cat.err_validation_short,
                commitkit::hooks::ValidationError::SubjectTooLong => cat.err_validation_long,
                commitkit::hooks::ValidationError::InvalidScope => cat.err_validation_scope,
            };

            Err(anyhow::anyhow!(error_message))
        }
    }
}

/// Prepare a commit message for the git hook
fn prepare_commit_message(message: &str, config: &Config) -> Result<()> {
    // If the message already follows the conventional format, don't modify it
    let re = regex::Regex::new(r"^(\w+)(\(([\w-]+)\))?: (.+)").unwrap();
    if re.is_match(message) {
        println!("{}", message);
        return Ok(());
    }

    // Otherwise, assume it's a regular message and format it as a conventional commit
    let prefixes = &config.prefixes;

    // Default to "chore" if available, otherwise first prefix
    let default_prefix = prefixes
        .iter()
        .find(|p| p.title == "chore")
        .unwrap_or(&prefixes[0]);

    let formatted = format!("{}: {}", default_prefix.title, message.trim());

    // Apply emoji if configured
    let final_message = if config.use_emoji {
        commitkit::emoji::apply_emoji(&default_prefix.title, &formatted, true)
    } else {
        formatted
    };

    println!("{}", final_message);
    Ok(())
}

/// Show commit statistics
fn show_commit_stats(args: &Args, locale: Locale) -> Result<()> {
    let cat = locale.catalog();
    let current_dir = std::env::current_dir()?;
    let repo_root = commitkit::hooks::GitHookManager::find_repo_root(&current_dir)
        .ok_or_else(|| anyhow::anyhow!(cat.err_not_git_repo))?;

    let analyzer = commitkit::stats::CommitAnalyzer::new(repo_root.to_str().unwrap_or("."));
    let summary = analyzer.get_type_summary_localized(args.days, locale)?;

    println!("{}", summary);
    Ok(())
}

/// Create a commit using a template
fn create_template_commit(
    template_name: &str,
    config: &Config,
    use_emoji: bool,
    update_changelog: bool,
    args: &Args,
    locale: Locale,
) -> Result<()> {
    let cat = locale.catalog();
    // Initialize the template manager
    let template_manager = commitkit::template::TemplateManager::new(&config.templates_dir)?;

    // Get the requested template
    let template = template_manager
        .get_template(template_name)
        .ok_or_else(|| anyhow::anyhow!(fmt1(cat.err_template_not_found, "name", template_name)))?;

    println!(
        "{}",
        fmt2(
            cat.msg_using_template,
            "name",
            &template.name,
            "description",
            &template.description
        )
    );

    // Collect values for the template
    let mut values = std::collections::HashMap::new();
    let prompter = TerminalPrompter::with_locale(locale);

    // Extract placeholders from templates
    let re = regex::Regex::new(r"\{([^}]+)\}").unwrap();

    // From subject template
    for cap in re.captures_iter(&template.subject_template) {
        let placeholder = cap[1].to_string();
        if let std::collections::hash_map::Entry::Vacant(e) = values.entry(placeholder) {
            let prompt = fmt1(cat.prompt_template_value, "name", e.key());
            let value = prompter.prompt_custom(&prompt)?;
            e.insert(value);
        }
    }

    // From body template if present
    if let Some(body_tpl) = &template.body_template {
        for cap in re.captures_iter(body_tpl) {
            let placeholder = cap[1].to_string();
            if let std::collections::hash_map::Entry::Vacant(e) = values.entry(placeholder) {
                let prompt = fmt1(cat.prompt_template_value, "name", e.key());
                let value = prompter.prompt_custom(&prompt)?;
                e.insert(value);
            }
        }
    }

    // From footer template if present
    if let Some(footer_tpl) = &template.footer_template {
        for cap in re.captures_iter(footer_tpl) {
            let placeholder = cap[1].to_string();
            if let std::collections::hash_map::Entry::Vacant(e) = values.entry(placeholder) {
                let prompt = fmt1(cat.prompt_template_value, "name", e.key());
                let value = prompter.prompt_custom(&prompt)?;
                e.insert(value);
            }
        }
    }

    // Fill the templates
    let subject = commitkit::template::fill_template(&template.subject_template, &values);
    let body = template
        .body_template
        .as_ref()
        .map(|tpl| commitkit::template::fill_template(tpl, &values))
        .unwrap_or_default();
    let footer = template
        .footer_template
        .as_ref()
        .map(|tpl| commitkit::template::fill_template(tpl, &values))
        .unwrap_or_default();

    // Determine prefix and scope
    let re_type = regex::Regex::new(r"^(\w+)(?:\(([\w-]+)\))?:").unwrap();
    let (prefix, scope) = if let Some(caps) = re_type.captures(&subject) {
        (
            caps.get(1)
                .map(|m| m.as_str())
                .unwrap_or("chore")
                .to_string(),
            caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string(),
        )
    } else {
        // Default to first prefix type if not found
        (config.prefixes[0].title.clone(), String::new())
    };

    // Clean subject of any type/scope prefix
    let clean_subject = re_type.replace(&subject, "").trim().to_string();

    // Build commit message with or without emoji
    let commit_message = if use_emoji {
        commitkit::build_commit_message_with_emoji(
            &prefix,
            &scope,
            &clean_subject,
            &body,
            &footer,
            true,
        )
    } else {
        commitkit::build_commit_message(&prefix, &scope, &clean_subject, &body, &footer)
    };

    // Handle dry run
    if args.dry_run {
        println!("{}", cat.dry_run_template_header);
        println!("{}", commit_message);
        println!("-----------------------------------");
        if args.verbose {
            println!("{}", cat.msg_dry_run_notice);
        }
        return Ok(());
    }

    // Execute the commit
    if args.verbose {
        println!("{}", cat.msg_executing_commit);
    }

    git::run_git_commit(&commit_message, config.sign_off_commits).context(cat.err_commit_failed)?;

    println!("{}", cat.msg_template_commit_success);

    // Update changelog if enabled
    if update_changelog {
        if args.verbose {
            println!("{}", cat.msg_updating_changelog);
        }

        let current_dir = std::env::current_dir()?;
        let changelog_path = current_dir.join("CHANGELOG.md");

        let changelog = commitkit::changelog::ChangelogManager::new(
            &changelog_path,
            &current_dir
                .file_name()
                .unwrap_or_default()
                .to_string_lossy(),
        );

        changelog
            .add_entry(&prefix, Some(&scope), &clean_subject, Some(&body))
            .context(cat.err_changelog_update)?;

        if args.verbose {
            println!("{}", cat.msg_changelog_updated);
        }
    }

    Ok(())
}
