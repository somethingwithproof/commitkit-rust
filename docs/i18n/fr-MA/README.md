[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit est un outil en ligne de commande qui accompagne les développeurs dans la rédaction de messages de commit structurés et homogènes selon la norme [Conventional Commits](https://www.conventionalcommits.org/). Son interface interactive vous guide étape par étape pour composer un message complet avec le type, une portée facultative, l'objet, le corps et le pied de page.

## Fonctionnalités

- **Fonctionnalités de base**
  - Choix interactif des types de commit et des portées
  - Configuration personnalisable via `.commitkit.toml`
  - Respect de la longueur maximale de l'objet
  - Prise en charge des corps de commit sur plusieurs lignes
  - Vérification automatique des saisies
  - Mention `Signed-off-by` facultative dans les commits
  - Compatibilité multiplateforme

- **Fonctionnalités avancées**
  - Prise en charge des émojis dans les messages de commit (✨, 🐛, etc.)
  - Modèles de commit avec remplacement de variables
  - Génération et mise à jour automatiques du journal des modifications
  - Hooks Git pour vérifier et mettre en forme les messages de commit
  - Statistiques et analyse de l'historique des commits
  - Saisie interactive des valeurs pour les modèles

## Prérequis

- **Rust** : version 1.88.0 ou supérieure (MSRV)
- **Cargo** : installé avec Rust

## Installation

### À partir du code source

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Utilisation

Accédez à votre dépôt Git puis lancez :

```shell
commitkit
```

### Options de la ligne de commande

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Ne pas exécuter git commit et afficher le message sur stdout
    -v, --verbose           Afficher les informations détaillées
    -c, --config            Chemin vers le fichier de configuration (par défaut : .commitkit.toml dans le dossier courant ou le dossier utilisateur)
        --emoji             Activer ou désactiver les émojis dans les messages de commit
        --template          Utiliser un modèle de commit précis
        --changelog         Mettre à jour le journal des modifications après le commit
        --install-hooks     Installer les hooks git
        --stats             Afficher les statistiques de commits
        --days              Nombre de jours à analyser pour les statistiques (par défaut : tous)
        --validate          Vérifier un fichier de message de commit
        --prepare-msg       Préparer le message de commit (invoqué par un hook git)
        --lang <LANG>       Langue de l'interface (ex. : en, fr-MA, ar-MA, fr-SN, ja)
    -h, --help              Afficher l'aide
    -V, --version           Afficher la version
```

### Langues disponibles

CommitKit détecte la langue d'affichage via `LC_ALL`, `LC_MESSAGES` et `LANG`, ou accepte `--lang <LANG>`. L'outil prend en charge 45 langues, dont 6 langues s'écrivant de droite à gauche (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` et `ur`). Retrouvez le tableau complet dans le [README en anglais](../../../README.md).

### Parcours interactif

L'assistant vous accompagne dans la création d'un message de commit conventionnel :

1. Choisissez un type de commit (`feat`, `fix`, `docs`, etc.)
2. Choisissez une portée (facultatif)
3. Entrez l'objet du commit
4. Entrez le corps du commit (facultatif)
5. Entrez le pied de page du commit (facultatif)

À la fin de ces étapes, CommitKit lance `git commit` avec le message préparé.

### Exemples

```shell
# Utilisation standard
commitkit

# Simulation - afficher le message généré sans enregistrer de commit
commitkit --dry-run

# Utiliser un fichier de configuration personnalisé
commitkit --config /path/to/custom-config.toml

# Informations détaillées
commitkit --verbose

# Activer les émojis dans les messages de commit
commitkit --emoji

# Utiliser un modèle pour le commit
commitkit --template feature

# Mettre à jour CHANGELOG.md automatiquement
commitkit --changelog

# Installer les hooks git
commitkit --install-hooks

# Afficher les statistiques des commits des 30 derniers jours
commitkit --stats --days 30

# Vérifier un message de commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Développement

### Compilation

```bash
# Compilation de débogage
cargo build

# Compilation release
cargo build --release
```

### Tests

```bash
# Lancer tous les tests
cargo test

# Lancer les tests avec sortie détaillée
cargo test -- --nocapture
```

### Vérification du code

```bash
# Contrôler les avertissements Clippy
cargo clippy --all-targets --all-features

# Appliquer les corrections automatiques
cargo clippy --fix --allow-dirty
```

### Mise en forme

```bash
# Vérifier le formatage
cargo fmt -- --check

# Appliquer le formatage
cargo fmt
```

## Configuration

CommitKit recherche un fichier `.commitkit.toml` dans l'ordre suivant :

1. Dossier courant
2. Dossier utilisateur

Si aucun fichier n'est détecté, un fichier par défaut est généré automatiquement dans votre dossier utilisateur.

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

Ce projet est distribué sous licence MIT — reportez-vous au fichier [LICENSE](../../../LICENSE) pour les détails.

## Contribution

Les contributions sont les bienvenues ! Vous pouvez ouvrir des issues ou soumettre des pull requests.

1. Forkez le dépôt
2. Créez votre branche de fonctionnalité (`git checkout -b feature/amazing-feature`)
3. Enregistrez vos commits au format Conventional Commits
4. Poussez la branche (`git push origin feature/amazing-feature`)
5. Ouvrez une Pull Request
