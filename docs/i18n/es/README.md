[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit es una herramienta de línea de comandos que ayuda a los desarrolladores a crear mensajes de commit bien estructurados y coherentes siguiendo la especificación [Conventional Commits](https://www.conventionalcommits.org/). Proporciona una interfaz interactiva que guía al usuario en la construcción de un mensaje de commit con tipo, ámbito opcional, asunto, cuerpo y pie.

## Características

- **Funcionalidad principal**
  - Selección interactiva de tipos de commit y ámbitos
  - Configuración personalizable mediante `.commitkit.toml`
  - Control de la longitud máxima del asunto
  - Compatibilidad con cuerpos de commit multilínea
  - Validación automática de la entrada
  - Línea `Signed-off-by` opcional en los commits
  - Compatibilidad multiplataforma

- **Funciones avanzadas**
  - Soporte de emojis en mensajes de commit (✨, 🐛, etc.)
  - Plantillas de commit con sustitución de marcadores de posición
  - Generación y mantenimiento automáticos del registro de cambios
  - Hooks de Git para validar y formatear mensajes de commit
  - Estadísticas y análisis de commits
  - Indicaciones interactivas para valores de plantilla

## Requisitos previos

- **Rust**: Versión 1.88.0 o superior (MSRV)
- **Cargo**: Incluido en la instalación de Rust

## Instalación

### Desde el código fuente

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Uso

Acceda a su repositorio Git y ejecute:

```shell
commitkit
```

### Opciones de línea de comandos

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Omitir git commit e imprimir el mensaje en stdout
    -v, --verbose           Mostrar salida detallada
    -c, --config            Ruta al archivo de configuración (predeterminado: .commitkit.toml en el directorio actual o personal)
        --emoji             Activar o desactivar emojis en los mensajes de commit
        --template          Usar una plantilla de commit específica
        --changelog         Actualizar el registro de cambios tras el commit
        --install-hooks     Instalar hooks de git
        --stats             Mostrar estadísticas de commits
        --days              Número de días a analizar para las estadísticas (predeterminado: todos)
        --validate          Validar un archivo de mensaje de commit
        --prepare-msg       Preparar mensaje de commit (usado por el hook de git)
        --lang <LANG>       Idioma de visualización (p. ej., en, de, ja, zh-CN, pt-BR)
    -h, --help              Mostrar información de ayuda
    -V, --version           Mostrar información de versión
```

### Idiomas admitidos

CommitKit detecta el idioma de visualización a partir de `LC_ALL`, `LC_MESSAGES` y `LANG`, o acepta `--lang <LANG>`. Admite 45 idiomas, incluidos 6 idiomas de derecha a izquierda (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` y `ur`). Consulte la tabla completa en el [README en inglés](../../../README.md).

### Proceso interactivo

La herramienta le guiará paso a paso para crear un mensaje de commit convencional:

1. Seleccionar un tipo de commit (`feat`, `fix`, `docs`, etc.)
2. Seleccionar un ámbito (opcional)
3. Introducir el asunto del commit
4. Introducir el cuerpo del commit (opcional)
5. Introducir el pie del commit (opcional)

Tras completar estos pasos, CommitKit ejecutará `git commit` con el mensaje construido.

### Ejemplos

```shell
# Uso básico
commitkit

# Simulación - mostrar el mensaje resultante sin hacer commit
commitkit --dry-run

# Usar un archivo de configuración personalizado
commitkit --config /path/to/custom-config.toml

# Salida detallada
commitkit --verbose

# Usar emojis en los mensajes de commit
commitkit --emoji

# Usar una plantilla para el commit
commitkit --template feature

# Actualizar CHANGELOG.md automáticamente
commitkit --changelog

# Instalar hooks de git
commitkit --install-hooks

# Mostrar estadísticas de commits de los últimos 30 días
commitkit --stats --days 30

# Validar un mensaje de commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Desarrollo

### Compilación

```bash
# Compilación de depuración
cargo build

# Compilación de lanzamiento
cargo build --release
```

### Pruebas

```bash
# Ejecutar todas las pruebas
cargo test

# Ejecutar pruebas mostrando la salida
cargo test -- --nocapture
```

### Análisis estático

```bash
# Comprobar problemas de estilo y análisis
cargo clippy --all-targets --all-features

# Aplicar correcciones automáticas
cargo clippy --fix --allow-dirty
```

### Formato

```bash
# Comprobar formato
cargo fmt -- --check

# Aplicar formato
cargo fmt
```

## Configuración

CommitKit busca un archivo de configuración llamado `.commitkit.toml` en las siguientes ubicaciones (en orden):

1. Directorio actual
2. Directorio personal del usuario

Si no se encuentra ningún archivo de configuración, se generará uno predeterminado en su directorio personal.

Ejemplo de configuración:

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

## Licencia

Este proyecto está bajo la licencia MIT; consulte el archivo [LICENSE](../../../LICENSE) para obtener más detalles.

## Contribuciones

Las contribuciones son bienvenidas. No dude en abrir incidencias o enviar solicitudes de extracción.

1. Haga un fork del repositorio
2. Cree su rama de funcionalidad (`git checkout -b feature/amazing-feature`)
3. Confirme sus cambios usando commits convencionales
4. Suba la rama (`git push origin feature/amazing-feature`)
5. Abra un Pull Request
