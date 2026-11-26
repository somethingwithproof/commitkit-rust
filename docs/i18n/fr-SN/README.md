[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit est un outil CLI qui aide les développeurs à composer des messages de commit clairs, structurés et conformes à la spécification [Conventional Commits](https://www.conventionalcommits.org/). Grâce à son mode interactif, il vous guide étape par étape pour renseigner le type, un périmètre optionnel, le titre, le corps et le pied du commit.

## Fonctionnalités

- **Fonctionnalités essentielles**
  - Sélection interactive des types de commit et des périmètres
  - Configuration personnalisable par `.commitkit.toml`
  - Contrôle de la longueur maximale du titre
  - Gestion des corps de commit multilignes
  - Contrôle automatique des entrées
  - Mention `Signed-off-by` facultative dans les commits
  - Compatibilité multiplateforme

- **Fonctionnalités complémentaires**
  - Gestion des émojis dans les messages de commit (✨, 🐛, etc.)
  - Modèles de commit avec substitution de paramètres
  - Génération et actualisation automatiques du journal des changements
  - Hooks Git pour contrôler et formater les messages de commit
  - Statistiques et analyse des commits
  - Invites interactives pour renseigner les modèles

## Prérequis

- **Rust** : version 1.88.0 ou plus récente (MSRV)
- **Cargo** : fourni avec Rust

## Installation

### Depuis le code source

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Utilisation

Ouvrez votre dépôt Git et exécutez :

```shell
commitkit
```

### Options en ligne de commande

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Ignorer git commit et écrire le message sur stdout
    -v, --verbose           Activer l'affichage détaillé
    -c, --config            Chemin du fichier de configuration (par défaut : .commitkit.toml dans le dossier actuel ou le dossier personnel)
        --emoji             Activer ou désactiver les émojis dans les messages de commit
        --template          Employer un modèle de commit spécifique
        --changelog         Actualiser le journal des changements après le commit
        --install-hooks     Installer les hooks git
        --stats             Consulter les statistiques des commits
        --days              Nombre de jours à couvrir pour les statistiques (par défaut : tous)
        --validate          Contrôler un fichier de message de commit
        --prepare-msg       Préparer le message de commit (appelé par un hook git)
        --lang <LANG>       Langue d'affichage (ex. : en, fr-SN, fr-MA, sw-KE, ja)
    -h, --help              Afficher l'aide
    -V, --version           Afficher la version
```

### Langues prises en charge

CommitKit identifie la langue d'affichage à partir de `LC_ALL`, `LC_MESSAGES` et `LANG`, ou accepte `--lang <LANG>`. Il prend en charge 45 langues, dont 6 langues s'écrivant de droite à gauche (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` et `ur`). Consultez la liste complète dans le [README en anglais](../../../README.md).

### Mode interactif

L'outil vous guide pas à pas pour élaborer un message de commit conventionnel :

1. Sélectionnez un type de commit (`feat`, `fix`, `docs`, etc.)
2. Sélectionnez un périmètre (optionnel)
3. Saisissez le titre du commit
4. Saisissez le corps du commit (optionnel)
5. Saisissez le pied du commit (optionnel)

Une fois la saisie achevée, CommitKit exécute `git commit` avec votre message.

### Exemples

```shell
# Utilisation de base
commitkit

# Mode simulation - afficher le message produit sans créer de commit
commitkit --dry-run

# Utiliser un fichier de configuration personnalisé
commitkit --config /path/to/custom-config.toml

# Affichage détaillé
commitkit --verbose

# Utiliser des émojis dans les messages de commit
commitkit --emoji

# Employer un modèle de commit
commitkit --template feature

# Actualiser CHANGELOG.md automatiquement
commitkit --changelog

# Installer les hooks git
commitkit --install-hooks

# Consulter les statistiques des commits des 30 derniers jours
commitkit --stats --days 30

# Contrôler un message de commit
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
# Exécuter l'ensemble des tests
cargo test

# Exécuter les tests avec affichage complet
cargo test -- --nocapture
```

### Analyse statique

```bash
# Vérifier les alertes Clippy
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

CommitKit recherche le fichier `.commitkit.toml` dans les emplacements ci-dessous (dans l'ordre) :

1. Dossier actuel
2. Dossier personnel de l'utilisateur

S'il n'existe aucun fichier de configuration, un fichier par défaut est créé dans votre dossier personnel.

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

Ce projet est placé sous licence MIT — consultez le fichier [LICENSE](../../../LICENSE) pour davantage d'informations.

## Contribution

Toute contribution est la bienvenue ! N'hésitez pas à ouvrir une issue ou à proposer une pull request.

1. Forkez le dépôt
2. Créez une branche dédiée (`git checkout -b feature/amazing-feature`)
3. Validez vos modifications avec des commits conventionnels
4. Poussez la branche (`git push origin feature/amazing-feature`)
5. Ouvrez une Pull Request
