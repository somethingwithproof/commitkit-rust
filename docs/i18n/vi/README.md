[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit là công cụ dòng lệnh giúp lập trình viên tạo các thông điệp commit có cấu trúc rõ ràng và nhất quán theo chuẩn [Conventional Commits](https://www.conventionalcommits.org/). Công cụ cung cấp giao diện tương tác hướng dẫn người dùng từng bước xây dựng thông điệp commit gồm loại, phạm vi tùy chọn, tiêu đề, nội dung và chân trang.

## Tính năng

- **Chức năng cốt lõi**
  - Lựa chọn tương tác loại commit và phạm vi
  - Cấu hình tùy chỉnh qua `.commitkit.toml`
  - Kiểm soát độ dài tối đa của tiêu đề
  - Hỗ trợ nội dung commit nhiều dòng
  - Tự động kiểm tra tính hợp lệ của dữ liệu nhập
  - Tùy chọn thêm dòng `Signed-off-by` vào commit
  - Tương thích đa nền tảng

- **Tính năng mở rộng**
  - Hỗ trợ emoji trong thông điệp commit (✨, 🐛, v.v.)
  - Mẫu commit có thay thế biến giữ chỗ
  - Tự động tạo và cập nhật nhật ký thay đổi
  - Git hook để kiểm tra và định dạng thông điệp commit
  - Thống kê và phân tích commit
  - Nhắc nhập tương tác cho các giá trị mẫu

## Yêu cầu hệ thống

- **Rust**: Phiên bản 1.88.0 trở lên (MSRV)
- **Cargo**: Đi kèm với bộ cài đặt Rust

## Cài đặt

### Từ mã nguồn

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Cách sử dụng

Di chuyển tới kho lưu trữ Git của bạn và chạy:

```shell
commitkit
```

### Tùy chọn dòng lệnh

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Bỏ qua git commit và in thông điệp ra stdout
    -v, --verbose           Hiển thị đầu ra chi tiết
    -c, --config            Đường dẫn tới tệp cấu hình (mặc định: .commitkit.toml trong thư mục hiện tại hoặc thư mục home)
        --emoji             Bật/tắt emoji trong thông điệp commit
        --template          Sử dụng mẫu commit cụ thể
        --changelog         Cập nhật nhật ký thay đổi sau khi commit
        --install-hooks     Cài đặt các git hook
        --stats             Hiển thị thống kê commit
        --days              Số ngày cần phân tích để thống kê (mặc định: tất cả)
        --validate          Kiểm tra tính hợp lệ của tệp thông điệp commit
        --prepare-msg       Chuẩn bị thông điệp commit (được git hook sử dụng)
        --lang <LANG>       Ngôn ngữ hiển thị (ví dụ: en, de, ja, zh-CN, pt-BR)
    -h, --help              Hiển thị thông tin trợ giúp
    -V, --version           Hiển thị thông tin phiên bản
```

### Ngôn ngữ được hỗ trợ

CommitKit tự động nhận diện ngôn ngữ hiển thị từ `LC_ALL`, `LC_MESSAGES` và `LANG`, hoặc nhận qua tham số `--lang <LANG>`. Công cụ hỗ trợ 45 ngôn ngữ, bao gồm 6 ngôn ngữ viết từ phải sang trái (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` và `ur`). Xem bảng đầy đủ trong [README tiếng Anh](../../../README.md).

### Quy trình tương tác

Công cụ sẽ hướng dẫn bạn từng bước tạo thông điệp commit theo chuẩn:

1. Chọn loại commit (`feat`, `fix`, `docs`, v.v.)
2. Chọn phạm vi (tùy chọn)
3. Nhập tiêu đề commit
4. Nhập nội dung commit (tùy chọn)
5. Nhập chân trang commit (tùy chọn)

Sau khi hoàn tất các bước này, CommitKit sẽ chạy `git commit` với thông điệp vừa tạo.

### Ví dụ

```shell
# Sử dụng cơ bản
commitkit

# Chạy thử - hiển thị thông điệp commit mà không tạo commit
commitkit --dry-run

# Dùng tệp cấu hình tùy chỉnh
commitkit --config /path/to/custom-config.toml

# Đầu ra chi tiết
commitkit --verbose

# Sử dụng emoji trong thông điệp commit
commitkit --emoji

# Dùng mẫu cho commit của bạn
commitkit --template feature

# Tự động cập nhật CHANGELOG.md
commitkit --changelog

# Cài đặt các git hook
commitkit --install-hooks

# Hiển thị thống kê commit trong 30 ngày qua
commitkit --stats --days 30

# Kiểm tra tính hợp lệ của thông điệp commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Phát triển

### Biên dịch

```bash
# Bản dựng gỡ lỗi
cargo build

# Bản dựng phát hành
cargo build --release
```

### Kiểm thử

```bash
# Chạy tất cả bài kiểm thử
cargo test

# Chạy kiểm thử kèm đầu ra
cargo test -- --nocapture
```

### Kiểm tra mã nguồn (Lint)

```bash
# Kiểm tra cảnh báo Clippy
cargo clippy --all-targets --all-features

# Áp dụng sửa lỗi tự động
cargo clippy --fix --allow-dirty
```

### Định dạng mã

```bash
# Kiểm tra định dạng
cargo fmt -- --check

# Áp dụng định dạng
cargo fmt
```

## Cấu hình

CommitKit tìm tệp cấu hình `.commitkit.toml` tại các vị trí sau (theo thứ tự):

1. Thư mục hiện tại
2. Thư mục home của người dùng

Nếu không tìm thấy tệp cấu hình, một tệp mặc định sẽ được tạo tự động trong thư mục home của bạn.

Cấu hình mẫu:

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

## Giấy phép

Dự án này được phát hành theo Giấy phép MIT — xem chi tiết tại tệp [LICENSE](../../../LICENSE).

## Đóng góp

Chúng tôi luôn hoan nghênh mọi đóng góp! Hãy mở issue hoặc gửi pull request.

1. Fork kho lưu trữ
2. Tạo nhánh tính năng (`git checkout -b feature/amazing-feature`)
3. Commit thay đổi bằng định dạng conventional commits
4. Đẩy nhánh lên (`git push origin feature/amazing-feature`)
5. Mở Pull Request
