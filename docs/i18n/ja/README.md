[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit は、[Conventional Commits](https://www.conventionalcommits.org/) 仕様に沿った構造化・統一されたコミットメッセージを作成するためのコマンドラインツールです。対話型インターフェースにより、タイプ、任意のスコープ、件名、本文、フッターを含むコミットメッセージの作成を段階的に案内します。

## 主な機能

- **基本機能**
  - コミットタイプおよびスコープの対話的な選択
  - `.commitkit.toml` による柔軟な設定
  - 件名の最大文字数制限の適用
  - 複数行のコミット本文のサポート
  - 入力内容の自動検証
  - コミットへの任意の `Signed-off-by` 行の追加
  - クロスプラットフォーム対応

- **拡張機能**
  - コミットメッセージでの絵文字サポート（✨、🐛 など）
  - プレースホルダー置換に対応したコミットテンプレート
  - チェンジログの自動生成および更新
  - コミットメッセージの検証・整形を行う Git フック
  - コミット統計の表示と分析
  - テンプレート値のための対話型プロンプト

## 動作要件

- **Rust**: バージョン 1.88.0 以上（MSRV）
- **Cargo**: Rust インストールに同梱

## インストール

### ソースからビルド

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## 使い方

Git リポジトリに移動して以下を実行します。

```shell
commitkit
```

### コマンドラインオプション

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit をスキップしてメッセージを標準出力に表示
    -v, --verbose           詳細出力を表示
    -c, --config            設定ファイルのパス (デフォルト: カレントまたはホームディレクトリの .commitkit.toml)
        --emoji             コミットメッセージの絵文字を切り替え
        --template          指定したコミットテンプレートを使用
        --changelog         コミット後にチェンジログを更新
        --install-hooks     git フックをインストール
        --stats             コミット統計を表示
        --days              統計を分析する日数 (デフォルト: すべて)
        --validate          コミットメッセージファイルを検証
        --prepare-msg       コミットメッセージを準備 (git フックが使用)
        --lang <LANG>       表示言語 (例: en, de, ja, zh-CN, pt-BR)
    -h, --help              ヘルプ情報を表示
    -V, --version           バージョン情報を表示
```

### 対応言語

CommitKit は `LC_ALL`、`LC_MESSAGES`、`LANG` 環境変数から表示言語を自動検出するほか、`--lang <LANG>` による明示的な指定にも対応しています。6 つの右横書き（RTL）言語（`ar`、`ar-EG`、`ar-MA`、`fa`、`he`、`ur`）を含む全 45 言語をサポートしています。全言語の一覧は[英語版 README](../../../README.md) をご覧ください。

### 対話型プロセス

ツールは Conventional Commits 形式のメッセージ作成を順を追って案内します。

1. コミットタイプを選択（`feat`、`fix`、`docs` など）
2. スコープを選択（任意）
3. コミットの件名を入力
4. コミットの本文を入力（任意）
5. コミットのフッターを入力（任意）

すべての手順が完了すると、CommitKit は生成されたメッセージで `git commit` を実行します。

### 使用例

```shell
# 基本的な使い方
commitkit

# ドライラン - コミットせずに生成されるメッセージを表示
commitkit --dry-run

# カスタム設定ファイルを使用
commitkit --config /path/to/custom-config.toml

# 詳細出力
commitkit --verbose

# コミットメッセージに絵文字を使用
commitkit --emoji

# テンプレートを使用してコミット
commitkit --template feature

# CHANGELOG.md を自動更新
commitkit --changelog

# git フックをインストール
commitkit --install-hooks

# 過去 30 日間のコミット統計を表示
commitkit --stats --days 30

# コミットメッセージを検証
commitkit --validate .git/COMMIT_EDITMSG
```

## 開発

### ビルド

```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

### テスト

```bash
# すべてのテストを実行
cargo test

# 標準出力を表示してテストを実行
cargo test -- --nocapture
```

### リント

```bash
# リントチェックを実行
cargo clippy --all-targets --all-features

# 自動修正を適用
cargo clippy --fix --allow-dirty
```

### フォーマット

```bash
# フォーマットを確認
cargo fmt -- --check

# フォーマットを適用
cargo fmt
```

## 設定

CommitKit は以下の順序で `.commitkit.toml` 設定ファイルを探索します。

1. カレントディレクトリ
2. ユーザーのホームディレクトリ

設定ファイルが見つからない場合、ホームディレクトリにデフォルトの設定ファイルが自動生成されます。

設定例:

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

## ライセンス

このプロジェクトは MIT ライセンスのもとで公開されています。詳細は [LICENSE](../../../LICENSE) ファイルをご覧ください。

## コントリビューション

コントリビューションを歓迎します。Issue の作成や Pull Request をお気軽にお送りください。

1. リポジトリをフォークする
2. 機能ブランチを作成する（`git checkout -b feature/amazing-feature`）
3. Conventional Commits 形式で変更をコミットする
4. ブランチをプッシュする（`git push origin feature/amazing-feature`）
5. Pull Request を作成する
