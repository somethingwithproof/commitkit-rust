// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use clap::{CommandFactory, FromArgMatches, Parser};
use std::ffi::OsString;
use std::process;

use commitkit::config::Config;
use commitkit::git;
use commitkit::i18n::{extract_cli_lang_os, fmt1, fmt2, Locale};
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
    let raw_args: Vec<OsString> = std::env::args_os().collect();
    let initial_locale = Locale::resolve(extract_cli_lang_os(&raw_args).as_deref());

    if let Err(err) = run(&raw_args, initial_locale) {
        let cat = initial_locale.catalog();
        eprintln!("{}", fmt1(cat.err_prefix, "error", &err.to_string()));
        process::exit(1);
    }
}

fn run(raw_args: &[OsString], initial_locale: Locale) -> Result<()> {
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

        let config_str = std::fs::read_to_string(&path)
            .with_context(|| fmt1(cat.err_config_read, "path", config_path))?;

        toml::from_str(&config_str)
            .with_context(|| fmt1(cat.err_config_parse, "path", config_path))?
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

    let prompter = TerminalPrompter::with_locale(locale);

    // Handle template-based commit if requested
    if let Some(template_name) = &args.template {
        return create_template_commit_with_prompter(
            &prompter,
            template_name,
            &config,
            use_emoji,
            update_changelog,
            &args,
            locale,
        );
    }

    run_interactive_commit(
        &prompter,
        &config,
        use_emoji,
        update_changelog,
        &args,
        locale,
    )
}

/// Run the standard interactive commit flow using the given prompter
fn run_interactive_commit(
    prompter: &dyn Prompter,
    config: &Config,
    use_emoji: bool,
    update_changelog: bool,
    args: &Args,
    locale: Locale,
) -> Result<()> {
    let cat = locale.catalog();

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

fn validation_error_message(
    err: commitkit::hooks::ValidationError,
    locale: Locale,
) -> &'static str {
    let cat = locale.catalog();
    match err {
        commitkit::hooks::ValidationError::InvalidFormat => cat.err_validation_format,
        commitkit::hooks::ValidationError::InvalidType => cat.err_validation_type,
        commitkit::hooks::ValidationError::SubjectTooShort => cat.err_validation_short,
        commitkit::hooks::ValidationError::SubjectTooLong => cat.err_validation_long,
        commitkit::hooks::ValidationError::InvalidScope => cat.err_validation_scope,
    }
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
        Err(err) => Err(anyhow::anyhow!(validation_error_message(err, locale))),
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
fn create_template_commit_with_prompter(
    prompter: &dyn Prompter,
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

#[cfg(test)]
mod tests {
    use super::*;
    use commitkit::config::Prefix;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::Mutex;
    use tempfile::TempDir;

    static CWD_LOCK: Mutex<()> = Mutex::new(());

    struct DirGuard {
        original: PathBuf,
    }

    impl DirGuard {
        fn enter(path: &Path) -> Self {
            let original = std::env::current_dir().expect("failed to get current dir");
            std::env::set_current_dir(path).expect("failed to change dir");
            Self { original }
        }
    }

    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.original);
        }
    }

    struct StubPrompter {
        prefix: String,
        scope: String,
        subject: String,
        body: String,
        footer: String,
    }

    impl StubPrompter {
        fn new() -> Self {
            Self {
                prefix: "feat".to_string(),
                scope: "cli".to_string(),
                subject: "add localized output".to_string(),
                body: "Body details".to_string(),
                footer: "Refs #1".to_string(),
            }
        }
    }

    impl Prompter for StubPrompter {
        fn prompt_prefix(&self, _prefixes: &[Prefix]) -> Result<String> {
            Ok(self.prefix.clone())
        }

        fn prompt_scope(&self, _scopes: &[String]) -> Result<String> {
            Ok(self.scope.clone())
        }

        fn prompt_subject(&self, _max_length: usize) -> Result<String> {
            Ok(self.subject.clone())
        }

        fn prompt_body(&self) -> Result<String> {
            Ok(self.body.clone())
        }

        fn prompt_footer(&self) -> Result<String> {
            Ok(self.footer.clone())
        }

        fn prompt_custom(&self, prompt: &str) -> Result<String> {
            Ok(format!("val({prompt})"))
        }
    }

    fn init_git_repo() -> TempDir {
        let dir = TempDir::new().expect("failed to create temp dir");
        assert!(Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .status()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(dir.path())
            .status()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(dir.path())
            .status()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .args(["config", "commit.gpgsign", "false"])
            .current_dir(dir.path())
            .status()
            .unwrap()
            .success());
        dir
    }

    fn os_args(slice: &[&str]) -> Vec<OsString> {
        slice.iter().map(OsString::from).collect()
    }

    fn test_config() -> Config {
        Config {
            sign_off_commits: false,
            prefixes: vec![
                Prefix {
                    title: "feat".to_string(),
                    description: "A new feature".to_string(),
                },
                Prefix {
                    title: "fix".to_string(),
                    description: "A bug fix".to_string(),
                },
                Prefix {
                    title: "chore".to_string(),
                    description: "Other changes".to_string(),
                },
            ],
            scopes: vec!["core".to_string(), "cli".to_string()],
            max_subject_len: 72,
            min_subject_len: 3,
            use_emoji: false,
            update_changelog: false,
            templates_dir: ".commitkit/templates".to_string(),
            validate_commit_msg: true,
        }
    }

    fn default_args(dry_run: bool, verbose: bool) -> Args {
        Args {
            dry_run,
            verbose,
            config: None,
            emoji: false,
            template: None,
            changelog: false,
            install_hooks: false,
            stats: false,
            days: None,
            validate: None,
            prepare_msg: None,
            lang: None,
        }
    }

    #[test]
    fn test_localized_command_help_de_and_ja() {
        for locale in [Locale::De, Locale::Ja] {
            let cat = locale.catalog();
            let mut cmd = localized_command(locale);
            let help = cmd.render_help().to_string();

            assert!(help.contains(cat.cli_about));
            assert!(help.contains(cat.cli_opt_dry_run));
            assert!(help.contains(cat.cli_opt_verbose));
            assert!(help.contains(cat.cli_opt_config));
            assert!(help.contains(cat.cli_opt_emoji));
            assert!(help.contains(cat.cli_opt_template));
            assert!(help.contains(cat.cli_opt_changelog));
            assert!(help.contains(cat.cli_opt_install_hooks));
            assert!(help.contains(cat.cli_opt_stats));
            assert!(help.contains(cat.cli_opt_days));
            assert!(help.contains(cat.cli_opt_validate));
            assert!(help.contains(cat.cli_opt_prepare_msg));
            assert!(help.contains(cat.cli_opt_lang));
        }
    }

    #[test]
    fn test_validate_commit_message_valid_and_all_errors() {
        let dir = TempDir::new().unwrap();
        let config = test_config();

        for locale in [Locale::En, Locale::De] {
            let cat = locale.catalog();

            let valid_file = dir.path().join("valid.txt");
            std::fs::write(&valid_file, "feat(core): add localized CLI messages\n").unwrap();
            assert!(validate_commit_message(valid_file.to_str().unwrap(), &config, locale).is_ok());

            let invalid_fmt = dir.path().join("invalid_fmt.txt");
            std::fs::write(&invalid_fmt, "not a conventional commit\n").unwrap();
            assert_eq!(
                validate_commit_message(invalid_fmt.to_str().unwrap(), &config, locale)
                    .unwrap_err()
                    .to_string(),
                cat.err_validation_format
            );

            let invalid_type = dir.path().join("invalid_type.txt");
            std::fs::write(&invalid_type, "unknown: valid length subject line\n").unwrap();
            assert_eq!(
                validate_commit_message(invalid_type.to_str().unwrap(), &config, locale)
                    .unwrap_err()
                    .to_string(),
                cat.err_validation_type
            );

            let too_short = dir.path().join("short.txt");
            std::fs::write(&too_short, "feat: hi\n").unwrap();
            assert_eq!(
                validate_commit_message(too_short.to_str().unwrap(), &config, locale)
                    .unwrap_err()
                    .to_string(),
                cat.err_validation_short
            );

            let too_long = dir.path().join("long.txt");
            std::fs::write(&too_long, format!("feat: {}\n", "a".repeat(120))).unwrap();
            assert_eq!(
                validate_commit_message(too_long.to_str().unwrap(), &config, locale)
                    .unwrap_err()
                    .to_string(),
                cat.err_validation_long
            );

            assert_eq!(
                validation_error_message(commitkit::hooks::ValidationError::InvalidScope, locale),
                cat.err_validation_scope
            );
        }
    }

    #[test]
    fn test_prepare_commit_message_variants() {
        let mut config = test_config();
        assert!(prepare_commit_message("feat(cli): already valid", &config).is_ok());
        assert!(prepare_commit_message("plain message", &config).is_ok());

        config.use_emoji = true;
        assert!(prepare_commit_message("plain message with emoji", &config).is_ok());
    }

    #[test]
    fn test_run_config_errors_and_cli_commands() {
        let _lock = CWD_LOCK.lock().unwrap();
        let temp = TempDir::new().unwrap();

        // 1. Nonexistent --config path
        let missing = temp.path().join("missing.toml");
        let missing_str = missing.to_str().unwrap();
        let err = run(
            &os_args(&["commitkit", "--lang", "de", "--config", missing_str]),
            Locale::De,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            fmt1(
                Locale::De.catalog().err_config_not_found,
                "path",
                missing_str
            )
        );

        // 2. Unreadable --config (directory instead of file)
        let dir_as_config = temp.path().to_str().unwrap();
        let err = run(
            &os_args(&["commitkit", "--lang", "de", "--config", dir_as_config]),
            Locale::De,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            fmt1(Locale::De.catalog().err_config_read, "path", dir_as_config)
        );

        // 3. Invalid TOML --config
        let bad_toml = temp.path().join("bad.toml");
        std::fs::write(&bad_toml, "not valid = [toml").unwrap();
        let bad_toml_str = bad_toml.to_str().unwrap();
        let err = run(
            &os_args(&["commitkit", "--lang", "ja", "--config", bad_toml_str]),
            Locale::Ja,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            fmt1(Locale::Ja.catalog().err_config_parse, "path", bad_toml_str)
        );

        // 4. Valid --config with --validate and --prepare-msg
        let good_cfg = temp.path().join("good.toml");
        std::fs::write(
            &good_cfg,
            r#"
sign_off_commits = false
prefixes = [
    { title = "feat", description = "A new feature" },
    { title = "chore", description = "Other changes" }
]
scopes = ["i18n"]
max_subject_len = 72
use_emoji = false
"#,
        )
        .unwrap();
        let good_cfg_str = good_cfg.to_str().unwrap();

        let msg_file = temp.path().join("COMMIT_EDITMSG");
        std::fs::write(&msg_file, "feat(i18n): validate localized message\n").unwrap();
        let msg_file_str = msg_file.to_str().unwrap();

        assert!(run(
            &os_args(&[
                "commitkit",
                "--lang",
                "fr",
                "--config",
                good_cfg_str,
                "--validate",
                msg_file_str,
            ]),
            Locale::Fr,
        )
        .is_ok());

        assert!(run(
            &os_args(&[
                "commitkit",
                "--config",
                good_cfg_str,
                "--prepare-msg",
                "update readme",
            ]),
            Locale::En,
        )
        .is_ok());

        // 5. Commands outside a git repo fail with localized err_not_git_repo
        let non_git = TempDir::new().unwrap();
        {
            let _cwd = DirGuard::enter(non_git.path());

            let err = run(
                &os_args(&["commitkit", "--lang", "de", "--install-hooks"]),
                Locale::De,
            )
            .unwrap_err();
            assert_eq!(err.to_string(), Locale::De.catalog().err_not_git_repo);

            let err = run(
                &os_args(&["commitkit", "--lang", "de", "--stats"]),
                Locale::De,
            )
            .unwrap_err();
            assert_eq!(err.to_string(), Locale::De.catalog().err_not_git_repo);

            let err = run(&os_args(&["commitkit", "--lang", "de"]), Locale::De).unwrap_err();
            assert_eq!(err.to_string(), Locale::De.catalog().err_not_git_repo);
        }

        // 6. Commands inside a git repo: install-hooks, stats, and no-staged-changes error
        let repo = init_git_repo();
        assert!(Command::new("git")
            .args([
                "commit",
                "--allow-empty",
                "-m",
                "feat(core): initial commit"
            ])
            .current_dir(repo.path())
            .status()
            .unwrap()
            .success());
        {
            let _cwd = DirGuard::enter(repo.path());

            assert!(run(
                &os_args(&["commitkit", "--lang", "de", "--install-hooks", "--verbose"]),
                Locale::De,
            )
            .is_ok());

            assert!(run(
                &os_args(&["commitkit", "--lang", "ja", "--stats", "--days", "30"]),
                Locale::Ja,
            )
            .is_ok());

            let err = run(&os_args(&["commitkit", "--lang", "es"]), Locale::Es).unwrap_err();
            assert_eq!(err.to_string(), Locale::Es.catalog().err_no_staged_changes);
        }
    }

    #[test]
    fn test_interactive_commit_dry_run_and_commit_with_changelog() {
        let _lock = CWD_LOCK.lock().unwrap();
        let config = test_config();
        let prompter = StubPrompter::new();

        // Dry run with verbose and emoji
        let dry_args = default_args(true, true);
        assert!(
            run_interactive_commit(&prompter, &config, true, false, &dry_args, Locale::De).is_ok()
        );
        assert!(
            run_interactive_commit(&prompter, &config, false, false, &dry_args, Locale::En).is_ok()
        );

        // Real commit + changelog update inside a temp git repo
        let repo = init_git_repo();
        std::fs::write(repo.path().join("file.txt"), "hello\n").unwrap();
        assert!(Command::new("git")
            .args(["add", "file.txt"])
            .current_dir(repo.path())
            .status()
            .unwrap()
            .success());

        {
            let _cwd = DirGuard::enter(repo.path());
            let commit_args = default_args(false, true);
            assert!(run_interactive_commit(
                &prompter,
                &config,
                false,
                true,
                &commit_args,
                Locale::Ja,
            )
            .is_ok());
            assert!(repo.path().join("CHANGELOG.md").exists());
        }
    }

    #[test]
    fn test_template_commit_missing_dry_run_and_commit() {
        let _lock = CWD_LOCK.lock().unwrap();
        let repo = init_git_repo();
        let tpl_dir = repo.path().join("templates");
        let mut config = test_config();
        config.templates_dir = tpl_dir.to_string_lossy().to_string();

        let prompter = StubPrompter::new();
        let dry_args = default_args(true, true);

        // Missing template returns localized error
        let err = create_template_commit_with_prompter(
            &prompter,
            "nonexistent",
            &config,
            false,
            false,
            &dry_args,
            Locale::De,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            fmt1(
                Locale::De.catalog().err_template_not_found,
                "name",
                "nonexistent"
            )
        );

        // Built-in "feature" template with subject, body, and footer placeholders (dry-run + emoji)
        assert!(create_template_commit_with_prompter(
            &prompter,
            "feature",
            &config,
            true,
            false,
            &dry_args,
            Locale::Fr,
        )
        .is_ok());

        // Real template commit + changelog update
        std::fs::write(repo.path().join("tpl.txt"), "template commit\n").unwrap();
        assert!(Command::new("git")
            .args(["add", "tpl.txt"])
            .current_dir(repo.path())
            .status()
            .unwrap()
            .success());

        {
            let _cwd = DirGuard::enter(repo.path());
            let commit_args = default_args(false, true);
            assert!(create_template_commit_with_prompter(
                &prompter,
                "feature",
                &config,
                false,
                true,
                &commit_args,
                Locale::En,
            )
            .is_ok());
            assert!(repo.path().join("CHANGELOG.md").exists());
        }
    }
}
