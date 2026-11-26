[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit — это инструмент командной строки, помогающий разработчикам создавать структурированные и единообразные сообщения коммитов в соответствии со спецификацией [Conventional Commits](https://www.conventionalcommits.org/). Он предоставляет интерактивный интерфейс, пошагово сопровождающий составление сообщения с типом, необязательной областью, заголовком, телом и нижним колонтитулом.

## Возможности

- **Основные функции**
  - Интерактивный выбор типов коммитов и областей
  - Настраиваемая конфигурация через `.commitkit.toml`
  - Контроль максимальной длины заголовка
  - Поддержка многострочного тела коммита
  - Автоматическая проверка вводимых данных
  - Необязательная строка `Signed-off-by` в коммитах
  - Кроссплатформенная совместимость

- **Расширенные функции**
  - Поддержка эмодзи в сообщениях коммитов (✨, 🐛 и др.)
  - Шаблоны коммитов с подстановкой параметров
  - Автоматическое создание и обновление журнала изменений
  - Хуки Git для проверки и форматирования сообщений коммитов
  - Статистика и анализ коммитов
  - Интерактивные запросы значений для шаблонов

## Требования

- **Rust**: версия 1.88.0 или выше (MSRV)
- **Cargo**: входит в установку Rust

## Установка

### Из исходного кода

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Использование

Перейдите в репозиторий Git и выполните:

```shell
commitkit
```

### Параметры командной строки

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Пропустить git commit и вывести сообщение в stdout
    -v, --verbose           Подробный вывод
    -c, --config            Путь к файлу конфигурации (по умолчанию: .commitkit.toml в текущем или домашнем каталоге)
        --emoji             Включить или отключить эмодзи в сообщениях коммитов
        --template          Использовать указанный шаблон коммита
        --changelog         Обновить журнал изменений после коммита
        --install-hooks     Установить хуки git
        --stats             Показать статистику коммитов
        --days              Количество дней для анализа статистики (по умолчанию: все)
        --validate          Проверить файл сообщения коммита
        --prepare-msg       Подготовить сообщение коммита (используется хуком git)
        --lang <LANG>       Язык интерфейса (напр., en, de, ja, zh-CN, pt-BR)
    -h, --help              Вывести справочную информацию
    -V, --version           Вывести информацию о версии
```

### Поддерживаемые языки

CommitKit определяет язык интерфейса по переменным `LC_ALL`, `LC_MESSAGES` и `LANG` либо принимает параметр `--lang <LANG>`. Поддерживается 45 языков, включая 6 языков с письмом справа налево (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` и `ur`). Полный список приведён в [английской версии README](../../../README.md).

### Интерактивный процесс

Инструмент проведёт вас через создание сообщения коммита в формате Conventional Commits:

1. Выберите тип коммита (`feat`, `fix`, `docs` и т. д.)
2. Выберите область (необязательно)
3. Введите заголовок коммита
4. Введите тело коммита (необязательно)
5. Введите нижний колонтитул коммита (необязательно)

После завершения этих шагов CommitKit выполнит `git commit` с подготовленным сообщением.

### Примеры

```shell
# Базовое использование
commitkit

# Пробный запуск — вывести итоговое сообщение без выполнения коммита
commitkit --dry-run

# Использование пользовательского файла конфигурации
commitkit --config /path/to/custom-config.toml

# Подробный вывод
commitkit --verbose

# Использование эмодзи в сообщениях коммитов
commitkit --emoji

# Использование шаблона коммита
commitkit --template feature

# Автоматическое обновление CHANGELOG.md
commitkit --changelog

# Установка хуков git
commitkit --install-hooks

# Статистика коммитов за последние 30 дней
commitkit --stats --days 30

# Проверка сообщения коммита
commitkit --validate .git/COMMIT_EDITMSG
```

## Разработка

### Сборка

```bash
# Отладочная сборка
cargo build

# Релизная сборка
cargo build --release
```

### Тестирование

```bash
# Запуск всех тестов
cargo test

# Запуск тестов с выводом
cargo test -- --nocapture
```

### Линтинг

```bash
# Проверка предупреждений Clippy
cargo clippy --all-targets --all-features

# Автоматическое исправление
cargo clippy --fix --allow-dirty
```

### Форматирование

```bash
# Проверка форматирования
cargo fmt -- --check

# Применение форматирования
cargo fmt
```

## Конфигурация

CommitKit ищет файл конфигурации `.commitkit.toml` в следующем порядке:

1. Текущий каталог
2. Домашний каталог пользователя

Если файл конфигурации не найден, в домашнем каталоге будет создан файл по умолчанию.

Пример конфигурации:

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

## Лицензия

Проект распространяется под лицензией MIT — подробности см. в файле [LICENSE](../../../LICENSE).

## Участие в разработке

Мы приветствуем вклад сообщества! Создавайте issues и отправляйте pull request.

1. Сделайте форк репозитория
2. Создайте ветку для функции (`git checkout -b feature/amazing-feature`)
3. Зафиксируйте изменения, используя Conventional Commits
4. Отправьте ветку (`git push origin feature/amazing-feature`)
5. Откройте Pull Request
