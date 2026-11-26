[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit è uno strumento da riga di comando che aiuta gli sviluppatori a creare messaggi di commit ben strutturati e coerenti secondo la specifica [Conventional Commits](https://www.conventionalcommits.org/). Offre un'interfaccia interattiva che guida l'utente nella composizione di un messaggio di commit con tipo, ambito facoltativo, oggetto, corpo e piè di pagina.

## Funzionalità

- **Funzionalità principali**
  - Selezione interattiva dei tipi di commit e degli ambiti
  - Configurazione personalizzabile tramite `.commitkit.toml`
  - Controllo della lunghezza massima dell'oggetto
  - Supporto per corpi di commit su più righe
  - Convalida automatica dell'input
  - Riga `Signed-off-by` facoltativa nei commit
  - Compatibilità multipiattaforma

- **Funzionalità avanzate**
  - Supporto emoji nei messaggi di commit (✨, 🐛, ecc.)
  - Modelli di commit con sostituzione dei segnaposto
  - Generazione e aggiornamento automatico del changelog
  - Hook Git per la convalida e la formattazione dei messaggi di commit
  - Statistiche e analisi dei commit
  - Richieste interattive per i valori dei modelli

## Prerequisiti

- **Rust**: versione 1.88.0 o successiva (MSRV)
- **Cargo**: incluso nell'installazione di Rust

## Installazione

### Dal codice sorgente

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Utilizzo

Spostarsi nel repository Git ed eseguire:

```shell
commitkit
```

### Opzioni da riga di comando

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Salta git commit e stampa il messaggio su stdout
    -v, --verbose           Mostra output dettagliato
    -c, --config            Percorso del file di configurazione (predefinito: .commitkit.toml nella directory corrente o home)
        --emoji             Attiva/disattiva emoji nei messaggi di commit
        --template          Usa un modello di commit specifico
        --changelog         Aggiorna il changelog dopo il commit
        --install-hooks     Installa gli hook git
        --stats             Mostra le statistiche dei commit
        --days              Numero di giorni da analizzare per le statistiche (predefinito: tutti)
        --validate          Convalida un file di messaggio di commit
        --prepare-msg       Prepara il messaggio di commit (usato dall'hook git)
        --lang <LANG>       Lingua di visualizzazione (es. en, de, ja, zh-CN, pt-BR)
    -h, --help              Stampa le informazioni di aiuto
    -V, --version           Stampa le informazioni sulla versione
```

### Lingue supportate

CommitKit rileva la lingua di visualizzazione da `LC_ALL`, `LC_MESSAGES` e `LANG`, oppure accetta `--lang <LANG>`. Supporta 45 lingue, tra cui 6 lingue con scrittura da destra a sinistra (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` e `ur`). L'elenco completo è disponibile nel [README in inglese](../../../README.md).

### Procedura interattiva

Lo strumento guida l'utente nella creazione di un messaggio di commit convenzionale:

1. Selezionare un tipo di commit (`feat`, `fix`, `docs`, ecc.)
2. Selezionare un ambito (facoltativo)
3. Inserire l'oggetto del commit
4. Inserire il corpo del commit (facoltativo)
5. Inserire il piè di pagina del commit (facoltativo)

Al termine di questi passaggi, CommitKit eseguirà `git commit` con il messaggio creato.

### Esempi

```shell
# Utilizzo di base
commitkit

# Simulazione - mostra il messaggio di commit senza effettuare il commit
commitkit --dry-run

# Usa un file di configurazione personalizzato
commitkit --config /path/to/custom-config.toml

# Output dettagliato
commitkit --verbose

# Usa le emoji nei messaggi di commit
commitkit --emoji

# Usa un modello per il commit
commitkit --template feature

# Aggiorna automaticamente CHANGELOG.md
commitkit --changelog

# Installa gli hook git
commitkit --install-hooks

# Mostra le statistiche dei commit negli ultimi 30 giorni
commitkit --stats --days 30

# Convalida un messaggio di commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Sviluppo

### Compilazione

```bash
# Compilazione debug
cargo build

# Compilazione release
cargo build --release
```

### Test

```bash
# Esegui tutti i test
cargo test

# Esegui i test mostrando l'output
cargo test -- --nocapture
```

### Linting

```bash
# Verifica problemi di linting
cargo clippy --all-targets --all-features

# Applica correzioni automatiche
cargo clippy --fix --allow-dirty
```

### Formattazione

```bash
# Verifica la formattazione
cargo fmt -- --check

# Applica la formattazione
cargo fmt
```

## Configurazione

CommitKit cerca un file di configurazione chiamato `.commitkit.toml` nei seguenti percorsi (in ordine):

1. Directory corrente
2. Directory home dell'utente

Se non viene trovato alcun file di configurazione, ne verrà generato uno predefinito nella directory home.

Configurazione di esempio:

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

## Licenza

Questo progetto è distribuito con licenza MIT: consulta il file [LICENSE](../../../LICENSE) per i dettagli.

## Contribuire

I contributi sono benvenuti! Apri pure issue o invia pull request.

1. Effettua il fork del repository
2. Crea il tuo branch di funzionalità (`git checkout -b feature/amazing-feature`)
3. Esegui il commit delle modifiche usando commit convenzionali
4. Invia il branch (`git push origin feature/amazing-feature`)
5. Apri una Pull Request
