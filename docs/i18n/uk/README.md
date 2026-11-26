[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit — це інструмент командного рядка, який допомагає розробникам створювати структуровані та послідовні повідомлення комітів відповідно до специфікації [Conventional Commits](https://www.conventionalcommits.org/). Він надає інтерактивний інтерфейс, що покроково супроводжує створення повідомлення з типом, необов'язковою областю, темою, основним текстом і нижнім колонтитулом.

## Можливості

- **Основні функції**
  - Інтерактивний вибір типів комітів і областей
  - Гнучка конфігурація через `.commitkit.toml`
  - Контроль максимальної довжини теми
  - Підтримка багаторядкового основного тексту коміту
  - Автоматична перевірка введених даних
  - Необов'язковий підпис комітів (`Signed-off-by`)
  - Кросплатформенна сумісність

- **Розширені функції**
  - Підтримка емодзі в повідомленнях комітів (✨, 🐛 тощо)
  - Шаблони комітів із підстановкою значень
  - Автоматичне формування та оновлення журналу змін
  - Хуки Git для перевірки й форматування повідомлень комітів
  - Статистика та аналіз комітів
  - Інтерактивні запити значень для шаблонів

## Вимоги

- **Rust**: версія 1.88.0 або вище (MSRV)
- **Cargo**: входить до складу Rust

## Встановлення

### Із вихідного коду

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Використання

Перейдіть у свій Git-репозиторій і виконайте:

```shell
commitkit
```

### Параметри командного рядка

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Пропустити git commit і вивести повідомлення до stdout
    -v, --verbose           Показувати докладний вивід
    -c, --config            Шлях до файлу конфігурації (за замовчуванням: .commitkit.toml у поточному або домашньому каталозі)
        --emoji             Увімкнути або вимкнути емодзі в повідомленнях комітів
        --template          Використати певний шаблон коміту
        --changelog         Оновити журнал змін після коміту
        --install-hooks     Встановити хуки git
        --stats             Показати статистику комітів
        --days              Кількість днів для аналізу статистики (за замовчуванням: усе)
        --validate          Перевірити файл повідомлення коміту
        --prepare-msg       Підготувати повідомлення коміту (використовується хуком git)
        --lang <LANG>       Мова інтерфейсу (напр., en, de, ja, zh-CN, pt-BR)
    -h, --help              Вивести довідкову інформацію
    -V, --version           Вивести інформацію про версію
```

### Підтримувані мови

CommitKit визначає мову інтерфейсу зі змінних `LC_ALL`, `LC_MESSAGES` і `LANG` або приймає `--lang <LANG>`. Підтримується 45 мов, зокрема 6 мов із письмом справа наліво (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` та `ur`). Повний перелік наведено в [англійському README](../../../README.md).

### Інтерактивний процес

Інструмент проведе вас через створення повідомлення коміту у форматі Conventional Commits:

1. Виберіть тип коміту (`feat`, `fix`, `docs` тощо)
2. Виберіть область (необов'язково)
3. Введіть тему коміту
4. Введіть основний текст коміту (необов'язково)
5. Введіть нижній колонтитул коміту (необов'язково)

Після завершення цих кроків CommitKit виконає `git commit` зі сформованим повідомленням.

### Приклади

```shell
# Базове використання
commitkit

# Пробний запуск — показати сформоване повідомлення без створення коміту
commitkit --dry-run

# Використання власного файлу конфігурації
commitkit --config /path/to/custom-config.toml

# Докладний вивід
commitkit --verbose

# Використання емодзі у повідомленнях комітів
commitkit --emoji

# Використання шаблону коміту
commitkit --template feature

# Автоматичне оновлення CHANGELOG.md
commitkit --changelog

# Встановлення хуків git
commitkit --install-hooks

# Статистика комітів за останні 30 днів
commitkit --stats --days 30

# Перевірка повідомлення коміту
commitkit --validate .git/COMMIT_EDITMSG
```

## Розробка

### Збирання

```bash
# Налагоджувальна збірка
cargo build

# Релізна збірка
cargo build --release
```

### Тестування

```bash
# Запустити всі тести
cargo test

# Запустити тести з виводом
cargo test -- --nocapture
```

### Лінтинг

```bash
# Перевірити зауваження Clippy
cargo clippy --all-targets --all-features

# Застосувати автоматичні виправлення
cargo clippy --fix --allow-dirty
```

### Форматування

```bash
# Перевірити форматування
cargo fmt -- --check

# Застосувати форматування
cargo fmt
```

## Конфігурація

CommitKit шукає файл конфігурації `.commitkit.toml` у такому порядку:

1. Поточний каталог
2. Домашній каталог користувача

Якщо файл конфігурації не знайдено, стандартний файл буде створено у вашому домашньому каталозі.

Приклад конфігурації:

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

## Ліцензія

Цей проєкт розповсюджується за ліцензією MIT — подробиці дивіться у файлі [LICENSE](../../../LICENSE).

## Внесок

Внески вітаються! Створюйте issues або надсилайте pull requests.

1. Зробіть форк репозиторію
2. Створіть гілку для нової функції (`git checkout -b feature/amazing-feature`)
3. Зафіксуйте зміни за допомогою Conventional Commits
4. Надішліть гілку (`git push origin feature/amazing-feature`)
5. Відкрийте Pull Request
