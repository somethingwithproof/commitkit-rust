[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

O CommitKit é uma ferramenta de linha de comando que ajuda desenvolvedores a criar mensagens de commit bem estruturadas e consistentes seguindo a especificação [Conventional Commits](https://www.conventionalcommits.org/). Ele oferece uma interface interativa que orienta o usuário na construção de uma mensagem de commit com tipo, escopo opcional, assunto, corpo e rodapé.

## Recursos

- **Funcionalidades principais**
  - Seleção interativa de tipos de commit e escopos
  - Configuração personalizável via `.commitkit.toml`
  - Controle do tamanho máximo do assunto
  - Suporte a corpos de commit com várias linhas
  - Validação automática das entradas
  - Commits assinados (`Signed-off-by`) opcionais
  - Compatibilidade multiplataforma

- **Recursos avançados**
  - Suporte a emojis nas mensagens de commit (✨, 🐛 etc.)
  - Modelos de commit com substituição de marcadores
  - Geração e atualização automática de changelog
  - Hooks do Git para validação e formatação de mensagens de commit
  - Estatísticas e análise de commits
  - Prompts interativos para valores de modelos

## Pré-requisitos

- **Rust**: versão 1.88.0 ou superior (MSRV)
- **Cargo**: incluído na instalação do Rust

## Instalação

### A partir do código-fonte

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Uso

Navegue até o seu repositório Git e execute:

```shell
commitkit
```

### Opções de linha de comando

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Pular git commit e exibir a mensagem na saída padrão
    -v, --verbose           Exibir saída detalhada
    -c, --config            Caminho para o arquivo de configuração (padrão: .commitkit.toml no diretório atual ou pessoal)
        --emoji             Ativar ou desativar emojis nas mensagens de commit
        --template          Usar um modelo de commit específico
        --changelog         Atualizar o changelog após o commit
        --install-hooks     Instalar hooks do git
        --stats             Exibir estatísticas de commits
        --days              Número de dias a analisar para estatísticas (padrão: todos)
        --validate          Validar um arquivo de mensagem de commit
        --prepare-msg       Preparar mensagem de commit (usado pelo hook do git)
        --lang <LANG>       Idioma de exibição (ex.: en, de, ja, zh-CN, pt-BR)
    -h, --help              Exibir informações de ajuda
    -V, --version           Exibir informações de versão
```

### Idiomas suportados

O CommitKit detecta o idioma de exibição a partir de `LC_ALL`, `LC_MESSAGES` e `LANG`, ou aceita `--lang <LANG>`. Há suporte para 45 idiomas, incluindo 6 idiomas da direita para a esquerda (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` e `ur`). Veja a tabela completa no [README em inglês](../../../README.md).

### Processo interativo

A ferramenta guiará você na criação de uma mensagem de commit convencional:

1. Selecionar um tipo de commit (`feat`, `fix`, `docs` etc.)
2. Selecionar um escopo (opcional)
3. Digitar o assunto do commit
4. Digitar o corpo do commit (opcional)
5. Digitar o rodapé do commit (opcional)

Após concluir essas etapas, o CommitKit executará `git commit` com a mensagem montada.

### Exemplos

```shell
# Uso básico
commitkit

# Simulação - exibe a mensagem resultante sem fazer commit
commitkit --dry-run

# Usar um arquivo de configuração personalizado
commitkit --config /path/to/custom-config.toml

# Saída detalhada
commitkit --verbose

# Usar emojis nas mensagens de commit
commitkit --emoji

# Usar um modelo para o commit
commitkit --template feature

# Atualizar CHANGELOG.md automaticamente
commitkit --changelog

# Instalar hooks do git
commitkit --install-hooks

# Exibir estatísticas de commits dos últimos 30 dias
commitkit --stats --days 30

# Validar uma mensagem de commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Desenvolvimento

### Compilação

```bash
# Compilação de depuração
cargo build

# Compilação de produção
cargo build --release
```

### Testes

```bash
# Executar todos os testes
cargo test

# Executar testes com saída detalhada
cargo test -- --nocapture
```

### Lint

```bash
# Verificar problemas de lint
cargo clippy --all-targets --all-features

# Aplicar correções automáticas
cargo clippy --fix --allow-dirty
```

### Formatação

```bash
# Verificar formatação
cargo fmt -- --check

# Aplicar formatação
cargo fmt
```

## Configuração

O CommitKit procura um arquivo de configuração chamado `.commitkit.toml` nos seguintes locais (em ordem):

1. Diretório atual
2. Diretório pessoal do usuário

Se nenhum arquivo de configuração for encontrado, um arquivo padrão será gerado no seu diretório pessoal.

Exemplo de configuração:

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

## Licença

Este projeto está licenciado sob a Licença MIT — consulte o arquivo [LICENSE](../../../LICENSE) para mais detalhes.

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou enviar pull requests.

1. Faça um fork do repositório
2. Crie sua branch de recurso (`git checkout -b feature/amazing-feature`)
3. Faça commit das alterações usando commits convencionais
4. Envie a branch (`git push origin feature/amazing-feature`)
5. Abra um Pull Request
