[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit은 개발자가 [Conventional Commits](https://www.conventionalcommits.org/) 명세에 따라 체계적이고 일관된 커밋 메시지를 작성할 수 있도록 돕는 명령줄 도구입니다. 유형, 선택적 범위, 제목, 본문, 푸터로 구성된 커밋 메시지를 단계별로 작성할 수 있는 대화형 인터페이스를 제공합니다.

## 기능

- **핵심 기능**
  - 커밋 유형 및 범위의 대화형 선택
  - `.commitkit.toml`을 통한 사용자 지정 설정
  - 제목 최대 길이 제한 적용
  - 여러 줄 커밋 본문 지원
  - 입력값 자동 유효성 검사
  - 커밋에 선택적 `Signed-off-by` 줄 추가
  - 크로스 플랫폼 호환성

- **확장 기능**
  - 커밋 메시지 내 이모지 지원(✨, 🐛 등)
  - 자리 표시자 치환을 지원하는 커밋 템플릿
  - 변경 로그 자동 생성 및 유지 관리
  - 커밋 메시지 검증 및 서식 지정을 위한 Git 훅
  - 커밋 통계 및 분석
  - 템플릿 값 입력을 위한 대화형 프롬프트

## 사전 요구 사항

- **Rust**: 버전 1.88.0 이상(MSRV)
- **Cargo**: Rust 설치에 포함됨

## 설치

### 소스에서 설치

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## 사용법

Git 저장소로 이동한 뒤 다음 명령을 실행하세요.

```shell
commitkit
```

### 명령줄 옵션

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit을 건너뛰고 메시지를 표준 출력으로 출력
    -v, --verbose           상세 출력 표시
    -c, --config            설정 파일 경로 (기본값: 현재 또는 홈 디렉터리의 .commitkit.toml)
        --emoji             커밋 메시지에서 이모지 사용 전환
        --template          지정한 커밋 템플릿 사용
        --changelog         커밋 후 변경 로그 업데이트
        --install-hooks     Git 훅 설치
        --stats             커밋 통계 표시
        --days              통계 분석 기간(일) (기본값: 전체)
        --validate          커밋 메시지 파일 유효성 검사
        --prepare-msg       커밋 메시지 준비 (git 훅에서 사용)
        --lang <LANG>       표시 언어 (예: en, de, ja, zh-CN, pt-BR)
    -h, --help              도움말 정보 출력
    -V, --version           버전 정보 출력
```

### 지원 언어

CommitKit은 `LC_ALL`, `LC_MESSAGES`, `LANG` 환경 변수에서 표시 언어를 감지하거나 `--lang <LANG>` 옵션을 지원합니다. 6개의 오른쪽에서 왼쪽으로 쓰는(RTL) 언어(`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, `ur`)를 포함하여 총 45개 언어를 지원합니다. 전체 목록은 [영문 README](../../../README.md)에서 확인하세요.

### 대화형 절차

도구가 컨벤셔널 커밋 메시지 작성 과정을 안내합니다.

1. 커밋 유형 선택(`feat`, `fix`, `docs` 등)
2. 범위 선택(선택 사항)
3. 커밋 제목 입력
4. 커밋 본문 입력(선택 사항)
5. 커밋 푸터 입력(선택 사항)

모든 단계를 완료하면 CommitKit이 작성된 메시지로 `git commit`을 실행합니다.

### 사용 예시

```shell
# 기본 사용법
commitkit

# 드라이 런 - 커밋하지 않고 생성된 커밋 메시지만 출력
commitkit --dry-run

# 사용자 지정 설정 파일 사용
commitkit --config /path/to/custom-config.toml

# 상세 출력
commitkit --verbose

# 커밋 메시지에 이모지 사용
commitkit --emoji

# 커밋에 템플릿 사용
commitkit --template feature

# CHANGELOG.md 자동 업데이트
commitkit --changelog

# git 훅 설치
commitkit --install-hooks

# 최근 30일간 커밋 통계 표시
commitkit --stats --days 30

# 커밋 메시지 유효성 검사
commitkit --validate .git/COMMIT_EDITMSG
```

## 개발

### 빌드

```bash
# 디버그 빌드
cargo build

# 릴리스 빌드
cargo build --release
```

### 테스트

```bash
# 모든 테스트 실행
cargo test

# 출력을 표시하며 테스트 실행
cargo test -- --nocapture
```

### 린트

```bash
# 린트 문제 확인
cargo clippy --all-targets --all-features

# 자동 수정 적용
cargo clippy --fix --allow-dirty
```

### 서식 지정

```bash
# 코드 서식 확인
cargo fmt -- --check

# 코드 서식 적용
cargo fmt
```

## 설정

CommitKit은 다음 순서로 `.commitkit.toml` 설정 파일을 검색합니다.

1. 현재 디렉터리
2. 사용자 홈 디렉터리

설정 파일을 찾지 못하면 홈 디렉터리에 기본 설정 파일이 자동으로 생성됩니다.

설정 예시:

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

## 라이선스

이 프로젝트는 MIT 라이선스로 배포됩니다. 자세한 내용은 [LICENSE](../../../LICENSE) 파일을 참고하세요.

## 기여하기

기여를 환영합니다! 이슈를 등록하거나 Pull Request를 제출해 주세요.

1. 저장소를 포크합니다
2. 기능 브랜치를 생성합니다(`git checkout -b feature/amazing-feature`)
3. 컨벤셔널 커밋 형식으로 변경 사항을 커밋합니다
4. 브랜치를 푸시합니다(`git push origin feature/amazing-feature`)
5. Pull Request를 엽니다
