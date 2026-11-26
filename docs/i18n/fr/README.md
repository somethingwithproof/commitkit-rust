[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit est un outil en ligne de commande qui aide les développeurs à créer des messages de commit bien structurés et cohérents selon la spécification [Conventional Commits](https://www.conventionalcommits.org/). Il offre une interface interactive guidant les utilisateurs étape par étape pour composer un message comprenant un type, une portée facultative, un sujet, un corps et un pied de message.

## Fonctionnalités

- **Fonctionnalités principales**
  - Sélection interactive des types de commit et des portées
  - Configuration personnalisable via `.commitkit.toml`
  - Contrôle de la longueur maximale du sujet
  - Prise en charge des corps de commit multilignes
  - Validation automatique des saisies
  - Mention `Signed-off-by` facultative dans les commits
  - Compatibilité multiplateforme

- **Fonctionnalités avancées**
  - Prise en charge des émojis dans les messages de commit (✨, 🐛, etc.)
  - Modèles de commit avec substitution de variables
  - Génération et mise à jour automatiques du changelog
  - Hooks Git pour la validation et le formatage des messages de commit
  - Statistiques et analyse des commits
  - Invites interactives pour les valeurs des modèles

## Prérequis

- **Rust** : version 1.88.0 ou supérieure (MSRV)
- **Cargo** : inclus avec l'installation de Rust

## Installation

### Depuis les sources

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Utilisation

Placez-vous dans votre dépôt Git et exécutez :

```shell
commitkit
```

### Options en ligne de commande

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Ignorer git commit et afficher le message sur stdout
    -v, --verbose           Afficher une sortie détaillée
    -c, --config            Chemin du fichier de configuration (par défaut : .commitkit.toml dans le répertoire courant ou personnel)
        --emoji             Activer/désactiver les émojis dans les messages de commit
        --template          Utiliser un modèle de commit spécifique
        --changelog         Mettre à jour le changelog après le commit
        --install-hooks     Installer les hooks git
        --stats             Afficher les statistiques des commits
        --days              Nombre de jours à analyser pour les statistiques (par défaut : tous)
        --validate          Valider un fichier de message de commit
        --prepare-msg       Préparer le message de commit (utilisé par un hook git)
        --lang <LANG>       Langue d'affichage (ex. : en, de, ja, zh-CN, pt-BR)
    -h, --help              Afficher l'aide
    -V, --version           Afficher la version
```

### Langues prises en charge

CommitKit détecte la langue d'affichage depuis `LC_ALL`, `LC_MESSAGES` et `LANG`, ou accepte `--lang <LANG>`. Il prend en charge 45 langues, dont 6 langues s'écrivant de droite à gauche (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` et `ur`). Consultez le tableau complet dans le [README en anglais](../../../README.md).

### Processus interactif

L'outil vous guide dans la création d'un message de commit conventionnel :

1. Sélectionner un type de commit (`feat`, `fix`, `docs`, etc.)
2. Sélectionner une portée (facultatif)
3. Saisir le sujet du commit
4. Saisir le corps du commit (facultatif)
5. Saisir le pied du commit (facultatif)

Une fois ces étapes terminées, CommitKit exécute `git commit` avec le message généré.

### Exemples

```shell
# Utilisation de base
commitkit

# Simulation - afficher le message de commit sans créer de commit
commitkit --dry-run

# Utiliser un fichier de configuration personnalisé
commitkit --config /path/to/custom-config.toml

# Sortie détaillée
commitkit --verbose

# Utiliser des émojis dans les messages de commit
commitkit --emoji

# Utiliser un modèle pour le commit
commitkit --template feature

# Mettre à jour CHANGELOG.md automatiquement
commitkit --changelog

# Installer les hooks git
commitkit --install-hooks

# Afficher les statistiques des commits des 30 derniers jours
commitkit --stats --days 30

# Valider un message de commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Développement

### Compilation

```bash
# Compilation de débogage
cargo build

# Compilation de production
cargo build --release
```

### Tests

```bash
# Exécuter tous les tests
cargo test

# Exécuter les tests avec sortie complète
cargo test -- --nocapture
```

### Analyse statique

```bash
# Vérifier les avertissements Clippy
cargo clippy --all-targets --all-features

# Appliquer les corrections automatiques
cargo clippy --fix --allow-dirty
```

### Formatage

```bash
# Vérifier le formatage
cargo fmt -- --check

# Appliquer le formatage
cargo fmt
```

## Configuration

CommitKit recherche un fichier de configuration nommé `.commitkit.toml` dans les emplacements suivants (dans l'ordre) :

1. Répertoire courant
2. Répertoire personnel de l'utilisateur

Si aucun fichier de configuration n'est trouvé, une configuration par défaut est créée dans votre répertoire personnel.

Exemple de configuration :

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

## Licence

Ce projet est distribué sous licence MIT — consultez le fichier [LICENSE](../../../LICENSE) pour plus de détails.

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir des issues ou à proposer des pull requests.

1. Forkez le dépôt
2. Créez votre branche de fonctionnalité (`git checkout -b feature/amazing-feature`)
3. Commitez vos modifications selon les commits conventionnels
4. Poussez la branche (`git push origin feature/amazing-feature`)
5. Ouvrez une Pull Request
