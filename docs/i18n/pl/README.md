[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit to narzędzie wiersza poleceń pomagające programistom tworzyć uporządkowane i spójne wiadomości commitów zgodnie ze specyfikacją [Conventional Commits](https://www.conventionalcommits.org/). Zapewnia interaktywny interfejs prowadzący krok po kroku przez konstruowanie wiadomości z typem, opcjonalnym zakresem, tematem, treścią i stopką.

## Funkcje

- **Podstawowa funkcjonalność**
  - Interaktywny wybór typów commitów i zakresów
  - Konfiguracja dostosowywana w pliku `.commitkit.toml`
  - Egzekwowanie maksymalnej długości tematu
  - Obsługa wielowierszowych treści commitów
  - Automatyczna walidacja danych wejściowych
  - Opcjonalne podpisywanie commitów (`Signed-off-by`)
  - Zgodność wieloplatformowa

- **Funkcje rozszerzone**
  - Obsługa emoji w wiadomościach commitów (✨, 🐛 itd.)
  - Szablony commitów z podstawianiem zmiennych
  - Automatyczne generowanie i aktualizacja dziennika zmian
  - Hooki Git do walidacji i formatowania wiadomości commitów
  - Statystyki i analiza commitów
  - Interaktywne pytania o wartości dla szablonów

## Wymagania wstępne

- **Rust**: wersja 1.88.0 lub nowsza (MSRV)
- **Cargo**: dołączone do instalacji Rusta

## Instalacja

### Ze źródeł

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Użycie

Przejdź do swojego repozytorium Git i uruchom:

```shell
commitkit
```

### Opcje wiersza poleceń

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Pomiń git commit i wypisz wiadomość na standardowe wyjście
    -v, --verbose           Pokaż szczegółowe wyjście
    -c, --config            Ścieżka do pliku konfiguracyjnego (domyślnie: .commitkit.toml w bieżącym lub domowym katalogu)
        --emoji             Przełącz emoji w wiadomościach commitów
        --template          Użyj konkretnego szablonu commita
        --changelog         Zaktualizuj dziennik zmian po zatwierdzeniu
        --install-hooks     Zainstaluj hooki git
        --stats             Pokaż statystyki commitów
        --days              Liczba dni do analizy statystyk (domyślnie: wszystkie)
        --validate          Zweryfikuj plik z wiadomością commita
        --prepare-msg       Przygotuj wiadomość commita (używane przez hook git)
        --lang <LANG>       Język wyświetlania (np. en, de, ja, zh-CN, pt-BR)
    -h, --help              Wyświetl informacje pomocy
    -V, --version           Wyświetl informacje o wersji
```

### Obsługiwane języki

CommitKit wykrywa język wyświetlania na podstawie zmiennych `LC_ALL`, `LC_MESSAGES` i `LANG` lub przyjmuje opcję `--lang <LANG>`. Obsługuje 45 języków, w tym 6 języków pisanych od prawej do lewej (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` i `ur`). Pełną tabelę znajdziesz w [angielskim pliku README](../../../README.md).

### Proces interaktywny

Narzędzie przeprowadzi Cię przez tworzenie konwencjonalnej wiadomości commita:

1. Wybierz typ commita (`feat`, `fix`, `docs` itd.)
2. Wybierz zakres (opcjonalnie)
3. Wpisz temat commita
4. Wpisz treść commita (opcjonalnie)
5. Wpisz stopkę commita (opcjonalnie)

Po zakończeniu tych kroków CommitKit wykona `git commit` z przygotowaną wiadomością.

### Przykłady

```shell
# Podstawowe użycie
commitkit

# Tryb próbny - wyświetl wygenerowaną wiadomość bez tworzenia commita
commitkit --dry-run

# Użycie własnego pliku konfiguracyjnego
commitkit --config /path/to/custom-config.toml

# Szczegółowe wyjście
commitkit --verbose

# Użycie emoji w wiadomościach commitów
commitkit --emoji

# Użycie szablonu commita
commitkit --template feature

# Automatyczna aktualizacja pliku CHANGELOG.md
commitkit --changelog

# Instalacja hooków git
commitkit --install-hooks

# Wyświetlenie statystyk commitów z ostatnich 30 dni
commitkit --stats --days 30

# Walidacja wiadomości commita
commitkit --validate .git/COMMIT_EDITMSG
```

## Rozwój

### Budowanie

```bash
# Kompilacja debugowa
cargo build

# Kompilacja wydaniowa
cargo build --release
```

### Testy

```bash
# Uruchom wszystkie testy
cargo test

# Uruchom testy z pełnym wyjściem
cargo test -- --nocapture
```

### Linting

```bash
# Sprawdź ostrzeżenia Clippy
cargo clippy --all-targets --all-features

# Zastosuj automatyczne poprawki
cargo clippy --fix --allow-dirty
```

### Formatowanie

```bash
# Sprawdź formatowanie
cargo fmt -- --check

# Zastosuj formatowanie
cargo fmt
```

## Konfiguracja

CommitKit wyszukuje plik konfiguracyjny `.commitkit.toml` w następujących lokalizacjach (w kolejności):

1. Bieżący katalog
2. Katalog domowy użytkownika

Jeśli plik konfiguracyjny nie zostanie znaleziony, w katalogu domowym zostanie utworzony plik domyślny.

Przykładowa konfiguracja:

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

## Licencja

Ten projekt jest udostępniany na licencji MIT — szczegóły znajdują się w pliku [LICENSE](../../../LICENSE).

## Współtworzenie

Wkład społeczności jest mile widziany! Zachęcamy do zgłaszania problemów i przesyłania pull requestów.

1. Zrób fork repozytorium
2. Utwórz gałąź dla funkcji (`git checkout -b feature/amazing-feature`)
3. Zatwierdź zmiany za pomocą konwencjonalnych commitów
4. Wypchnij gałąź (`git push origin feature/amazing-feature`)
5. Otwórz Pull Request
