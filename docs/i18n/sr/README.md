[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit је алат командне линије који помаже програмерима да пишу добро структурисане и доследне комит поруке у складу са спецификацијом [Conventional Commits](https://www.conventionalcommits.org/). Пружа интерактивни интерфејс који води кориснике кроз састављање комит поруке са типом, опционим опсегом, насловом, телом и подножјем.

## Функције

- **Основне функционалности**
  - Интерактивни избор типова комита и опсега
  - Прилагодљива подешавања преко датотеке `.commitkit.toml`
  - Контрола максималне дужине наслова
  - Подршка за вишередна тела комита
  - Аутоматска провера уноса
  - Опциона `Signed-off-by` линија у комитовима
  - Вишеплатформска компатибилност

- **Напредне функције**
  - Подршка за емоџије у комит порукама (✨, 🐛 итд.)
  - Шаблони за комит са заменом параметара
  - Аутоматско генерисање и одржавање дневника промена
  - Git куке (hooks) за проверу и форматирање комит порука
  - Статистика и анализа комитова
  - Интерактивни упити за вредности у шаблонима

## Предуслови

- **Rust**: верзија 1.88.0 или новија (MSRV)
- **Cargo**: укључен уз инсталацију Rust-а

## Инсталација

### Из изворног кода

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Коришћење

Отворите свој git репозиторијум и покрените:

```shell
commitkit
```

### Опције командне линије

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Skip git commit and print the message to stdout
    -v, --verbose           Show verbose output
    -c, --config            Path to config file (default: .commitkit.toml in current or home directory)
        --emoji             Toggle emoji in commit messages
        --template          Use a specific commit template
        --changelog         Update changelog after committing
        --install-hooks     Install git hooks
        --stats             Show commit statistics
        --days              Number of days to analyze for stats (default: all)
        --validate          Validate a commit message file
        --prepare-msg       Prepare commit message (used by git hook)
        --lang <LANG>       Display language (e.g. en, de, ja, zh-CN, pt-BR)
    -h, --help              Print help information
    -V, --version           Print version information
```

### Подржани језици

CommitKit препознаје језик приказа из променљивих окружења `LC_ALL`, `LC_MESSAGES` и `LANG`, или прихвата опцију `--lang <LANG>`. Подржава 48 језика (укључујући 6 језика писаних здесна налево: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` и `ur`). Потпуну табелу погледајте у [енглеској датотеци README](../../../README.md).

### Интерактивни поступак

Алат вас води кроз креирање конвенционалне комит поруке:

1. Изаберите тип комита (feat, fix, docs итд.)
2. Изаберите опсег (опционо)
3. Унесите наслов комита
4. Унесите тело комита (опционо)
5. Унесите подножје комита (опционо)

По завршетку ових корака, CommitKit покреће `git commit` са вашом састављеном поруком.

### Примери

```shell
# Basic usage
commitkit

# Dry run - display the resulting commit message without committing
commitkit --dry-run

# Use a custom config file
commitkit --config /path/to/custom-config.toml

# Verbose output
commitkit --verbose

# Use emojis in commit messages
commitkit --emoji

# Use a template for your commit
commitkit --template feature

# Automatically update CHANGELOG.md
commitkit --changelog

# Install git hooks
commitkit --install-hooks

# Show commit statistics for the last 30 days
commitkit --stats --days 30

# Validate a commit message
commitkit --validate .git/COMMIT_EDITMSG
```

## Развој

### Изградња

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Тестирање

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Провера кода (Lint)

```bash
# Check for linting issues
cargo clippy --all-targets --all-features

# Apply automatic fixes
cargo clippy --fix --allow-dirty
```

### Форматирање

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Подешавања

CommitKit тражи датотеку подешавања под називом `.commitkit.toml` на следећим локацијама (по редоследу):

1. Тренутни директоријум
2. Кориснички почетни директоријум

Ако датотека подешавања није пронађена, подразумевана датотека ће бити генерисана у вашем почетном директоријуму.

Пример подешавања:

```toml
# CommitKit Configuration

# Whether to add a Signed-off-by line to commits (-s flag)
sign_off_commits = false

# Commit message prefixes following Conventional Commits specification
prefixes = [
    { title = "feat", description = "A new feature" },
    { title = "fix", description = "A bug fix" },
    { title = "docs", description = "Documentation changes" },
    { title = "style", description = "Changes that do not affect code meaning" },
    { title = "refactor", description = "Code change that neither fixes a bug nor adds a feature" },
    { title = "perf", description = "Code change that improves performance" },
    { title = "test", description = "Adding missing tests or correcting existing tests" },
    { title = "build", description = "Changes that affect the build system or external dependencies" },
    { title = "ci", description = "Changes to CI configuration files and scripts" },
    { title = "chore", description = "Other changes that don't modify src or test files" },
    { title = "revert", description = "Reverts a previous commit" }
]

# Optional scopes to categorize changes
scopes = [
    "core",
    "ui",
    "docs",
    "tests",
    "deps"
]

# Subject line length constraints
max_subject_len = 72
min_subject_len = 3

# Feature toggles
use_emoji = true                      # Add emojis to commit messages
update_changelog = true               # Update CHANGELOG.md automatically
validate_commit_msg = true            # Validate commit messages against conventional format

# Templates directory for custom commit templates
templates_dir = "~/.commitkit/templates"
```

## Лиценца

Овај пројекат је лиценциран под MIT лиценцом - за детаље погледајте датотеку [LICENSE](../../../LICENSE).

## Доприноси

Доприноси су добродошли! Слободно отворите пријаве проблема или пошаљите захтеве за спајање (Pull Request).

1. Направите fork репозиторијума
2. Креирајте своју грану за функцију (`git checkout -b feature/amazing-feature`)
3. Сачувајте измене помоћу конвенционалних комит порука
4. Пошаљите грану на сервер (`git push origin feature/amazing-feature`)
5. Отворите Pull Request
