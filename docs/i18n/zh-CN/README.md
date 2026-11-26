[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit 是一款命令行工具，帮助开发者遵循 [Conventional Commits](https://www.conventionalcommits.org/)（约定式提交）规范编写结构清晰、风格统一的提交信息。它提供交互式界面，逐步引导用户构造包含类型、可选范围、简述、正文和脚注的提交信息。

## 功能特性

- **核心功能**
  - 交互式选择提交类型和影响范围
  - 通过 `.commitkit.toml` 进行灵活配置
  - 限制提交简述的最大长度
  - 支持多行提交正文
  - 自动校验输入内容
  - 可选在提交中添加 `Signed-off-by` 尾注
  - 跨平台兼容

- **增强功能**
  - 提交信息支持表情符号（✨、🐛 等）
  - 支持占位符替换的提交模板
  - 自动生成并维护变更日志
  - 用于校验和格式化提交信息的 Git 钩子
  - 提交统计与分析
  - 模板变量的交互式输入提示

## 环境要求

- **Rust**：1.88.0 或更高版本（MSRV）
- **Cargo**：随 Rust 安装提供

## 安装

### 从源码编译安装

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## 使用方法

进入您的 Git 仓库并运行：

```shell
commitkit
```

### 命令行选项

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           跳过 git commit 并将提交信息输出到标准输出
    -v, --verbose           显示详细输出
    -c, --config            配置文件路径（默认：当前目录或主目录下的 .commitkit.toml）
        --emoji             在提交信息中切换表情符号
        --template          使用指定的提交模板
        --changelog         提交后更新变更日志
        --install-hooks     安装 git 钩子
        --stats             显示提交统计信息
        --days              统计分析的天数（默认：全部）
        --validate          校验提交信息文件
        --prepare-msg       准备提交信息（由 git 钩子调用）
        --lang <LANG>       显示语言（例如 en、de、ja、zh-CN、pt-BR）
    -h, --help              打印帮助信息
    -V, --version           打印版本信息
```

### 支持的语言

CommitKit 会从 `LC_ALL`、`LC_MESSAGES` 和 `LANG` 自动检测显示语言，也支持通过 `--lang <LANG>` 手动指定。目前共支持 45 种语言（包括 6 种从右向左书写的语言：`ar`、`ar-EG`、`ar-MA`、`fa`、`he` 和 `ur`）。完整语言列表请参阅[英文 README](../../../README.md)。

### 交互式流程

该工具将引导您完成约定式提交信息的编写：

1. 选择提交类型（`feat`、`fix`、`docs` 等）
2. 选择影响范围（可选）
3. 输入提交简述
4. 输入提交正文（可选）
5. 输入提交脚注（可选）

完成以上步骤后，CommitKit 将使用生成的提交信息执行 `git commit`。

### 示例

```shell
# 基本用法
commitkit

# 试运行 - 仅打印生成的提交信息而不执行提交
commitkit --dry-run

# 使用自定义配置文件
commitkit --config /path/to/custom-config.toml

# 详细输出
commitkit --verbose

# 在提交信息中使用表情符号
commitkit --emoji

# 使用指定模板进行提交
commitkit --template feature

# 自动更新 CHANGELOG.md
commitkit --changelog

# 安装 git 钩子
commitkit --install-hooks

# 查看最近 30 天的提交统计
commitkit --stats --days 30

# 校验提交信息文件
commitkit --validate .git/COMMIT_EDITMSG
```

## 开发

### 构建

```bash
# 调试构建
cargo build

# 发布构建
cargo build --release
```

### 测试

```bash
# 运行所有测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture
```

### 代码检查

```bash
# 检查代码规范与潜在问题
cargo clippy --all-targets --all-features

# 应用自动修复
cargo clippy --fix --allow-dirty
```

### 代码格式化

```bash
# 检查格式
cargo fmt -- --check

# 应用格式化
cargo fmt
```

## 配置

CommitKit 按以下顺序查找名为 `.commitkit.toml` 的配置文件：

1. 当前目录
2. 用户主目录

如果未找到配置文件，将在主目录中自动生成一份默认配置。

配置示例：

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

## 许可证

本项目基于 MIT 许可证开源，详情请参阅 [LICENSE](../../../LICENSE) 文件。

## 贡献指南

欢迎参与贡献！您可以提交 Issue 或 Pull Request。

1. Fork 本仓库
2. 创建特性分支（`git checkout -b feature/amazing-feature`）
3. 使用约定式提交规范提交更改
4. 推送分支（`git push origin feature/amazing-feature`）
5. 提交 Pull Request
