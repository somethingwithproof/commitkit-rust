[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit 是一款命令列工具，協助開發者依照 [Conventional Commits](https://www.conventionalcommits.org/)（慣例式提交）規範建立結構清晰且一致的提交訊息。它提供互動式介面，逐步引導使用者撰寫包含類型、選填影響範圍、主旨、內文與註腳的提交訊息。

## 功能特色

- **核心功能**
  - 互動式選擇提交類型與影響範圍
  - 透過 `.commitkit.toml` 自訂設定
  - 強制執行主旨最大字元長度限制
  - 支援多行提交內文
  - 自動驗證輸入內容
  - 可選的簽署提交（`Signed-off-by`）
  - 跨平台相容

- **進階功能**
  - 提交訊息支援表情符號（✨、🐛 等）
  - 支援預留位置取代的提交範本
  - 自動產生與維護變更日誌
  - 用於驗證與格式化提交訊息的 Git hooks
  - 提交統計與分析
  - 範本數值的互動式輸入提示

## 先決條件

- **Rust**：1.88.0 或以上版本（MSRV）
- **Cargo**：隨 Rust 安裝附帶

## 安裝

### 從原始碼安裝

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## 使用方式

切換至您的 Git 儲存庫並執行：

```shell
commitkit
```

### 命令列選項

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           跳過 git commit 並將訊息輸出至標準輸出
    -v, --verbose           顯示詳細輸出
    -c, --config            設定檔路徑（預設：目前目錄或家目錄中的 .commitkit.toml）
        --emoji             切換提交訊息中的表情符號
        --template          使用指定的提交範本
        --changelog         提交後更新變更日誌
        --install-hooks     安裝 git hooks
        --stats             顯示提交統計資訊
        --days              要分析統計的天數（預設：全部）
        --validate          驗證提交訊息檔案
        --prepare-msg       準備提交訊息（由 git hook 使用）
        --lang <LANG>       顯示語言（例如 en、de、ja、zh-CN、pt-BR）
    -h, --help              列印說明資訊
    -V, --version           列印版本資訊
```

### 支援的語言

CommitKit 會從 `LC_ALL`、`LC_MESSAGES` 與 `LANG` 自動偵測顯示語言，也可透過 `--lang <LANG>` 手動指定。支援 45 種語言（包含 6 種由右至左書寫的語言：`ar`、`ar-EG`、`ar-MA`、`fa`、`he` 與 `ur`）。完整語言清單請參閱[英文版 README](../../../README.md)。

### 互動式流程

本工具會逐步引導您建立慣例式提交訊息：

1. 選擇提交類型（`feat`、`fix`、`docs` 等）
2. 選擇影響範圍（選填）
3. 輸入提交主旨
4. 輸入提交內文（選填）
5. 輸入提交註腳（選填）

完成這些步驟後，CommitKit 將以組裝好的訊息執行 `git commit`。

### 範例

```shell
# 基本用法
commitkit

# 模擬執行 - 顯示產生的提交訊息但不實際提交
commitkit --dry-run

# 使用自訂設定檔
commitkit --config /path/to/custom-config.toml

# 詳細輸出
commitkit --verbose

# 在提交訊息中使用表情符號
commitkit --emoji

# 使用範本進行提交
commitkit --template feature

# 自動更新 CHANGELOG.md
commitkit --changelog

# 安裝 git hooks
commitkit --install-hooks

# 顯示過去 30 天的提交統計資訊
commitkit --stats --days 30

# 驗證提交訊息
commitkit --validate .git/COMMIT_EDITMSG
```

## 開發

### 建置

```bash
# 偵錯建置
cargo build

# 發行建置
cargo build --release
```

### 測試

```bash
# 執行所有測試
cargo test

# 執行測試並顯示輸出
cargo test -- --nocapture
```

### 程式碼檢查

```bash
# 檢查 Clippy 警告
cargo clippy --all-targets --all-features

# 套用自動修正
cargo clippy --fix --allow-dirty
```

### 格式化

```bash
# 檢查格式
cargo fmt -- --check

# 套用格式化
cargo fmt
```

## 設定

CommitKit 會依序於下列位置尋找名為 `.commitkit.toml` 的設定檔：

1. 目前目錄
2. 使用者家目錄

若未找到設定檔，系統會自動於家目錄建立一份預設設定檔。

設定範例：

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

## 授權條款

本專案採用 MIT 授權條款，詳情請參閱 [LICENSE](../../../LICENSE) 檔案。

## 參與貢獻

歡迎各種貢獻！歡迎建立 Issue 或提交 Pull Request。

1. Fork 此儲存庫
2. 建立功能分支（`git checkout -b feature/amazing-feature`）
3. 使用慣例式提交格式提交變更
4. 推送分支（`git push origin feature/amazing-feature`）
5. 開啟 Pull Request
