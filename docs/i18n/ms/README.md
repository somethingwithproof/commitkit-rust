[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit ialah alat baris perintah (CLI) yang membantu pembangun mencipta mesej commit yang tersusun dan konsisten mengikut spesifikasi [Conventional Commits](https://www.conventionalcommits.org/). Ia menyediakan antara muka interaktif yang membimbing pengguna langkah demi langkah untuk membina mesej commit dengan jenis, skop pilihan, subjek, isi kandungan dan pengaki.

## Ciri-ciri

- **Fungsi Teras**
  - Pemilihan jenis commit dan skop secara interaktif
  - Konfigurasi boleh suai melalui `.commitkit.toml`
  - Penguatkuasaan had panjang maksimum subjek
  - Sokongan isi kandungan commit berbilang baris
  - Pengesahan input secara automatik
  - Pilihan commit bertandatangan (`Signed-off-by`)
  - Keserasian merentas platform

- **Ciri Lanjutan**
  - Sokongan emoji dalam mesej commit (✨, 🐛 dan sebagainya)
  - Templat commit dengan penggantian ruang letak
  - Penjanaan dan pengemaskinian log perubahan secara automatik
  - Hook Git untuk pengesahan dan pemformatan mesej commit
  - Statistik dan analisis commit
  - Gesaan interaktif untuk nilai templat

## Prasyarat

- **Rust**: Versi 1.88.0 atau lebih baharu (MSRV)
- **Cargo**: Disertakan bersama pemasangan Rust

## Pemasangan

### Daripada kod sumber

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Penggunaan

Buka repositori Git anda dan jalankan:

```shell
commitkit
```

### Pilihan baris perintah

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Langkau git commit dan cetak mesej ke stdout
    -v, --verbose           Papar output terperinci
    -c, --config            Laluan fail konfigurasi (lalai: .commitkit.toml dalam direktori semasa atau rumah)
        --emoji             Togol emoji dalam mesej commit
        --template          Gunakan templat commit tertentu
        --changelog         Kemas kini log perubahan selepas commit
        --install-hooks     Pasang hook git
        --stats             Papar statistik commit
        --days              Bilangan hari untuk dianalisis bagi statistik (lalai: semua)
        --validate          Sahkan fail mesej commit
        --prepare-msg       Sediakan mesej commit (digunakan oleh hook git)
        --lang <LANG>       Bahasa paparan (cth. en, ms, id, th, ja)
    -h, --help              Cetak maklumat bantuan
    -V, --version           Cetak maklumat versi
```

### Bahasa yang Disokong

CommitKit mengesan bahasa paparan daripada `LC_ALL`, `LC_MESSAGES` dan `LANG`, atau menerima `--lang <LANG>`. Ia menyokong 45 bahasa, termasuk 6 bahasa kanan-ke-kiri (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` dan `ur`). Rujuk jadual lengkap dalam [README Bahasa Inggeris](../../../README.md).

### Proses interaktif

Alat ini akan membimbing anda dalam mencipta mesej commit konvensional:

1. Pilih jenis commit (`feat`, `fix`, `docs` dan sebagainya)
2. Pilih skop (pilihan)
3. Masukkan subjek commit
4. Masukkan isi kandungan commit (pilihan)
5. Masukkan pengaki commit (pilihan)

Selepas melengkapkan langkah ini, CommitKit akan melaksanakan `git commit` dengan mesej yang telah dibina.

### Contoh

```shell
# Penggunaan asas
commitkit

# Mod simulasi - papar mesej commit tanpa melakukan commit
commitkit --dry-run

# Gunakan fail konfigurasi tersuai
commitkit --config /path/to/custom-config.toml

# Output terperinci
commitkit --verbose

# Gunakan emoji dalam mesej commit
commitkit --emoji

# Gunakan templat untuk commit anda
commitkit --template feature

# Kemas kini CHANGELOG.md secara automatik
commitkit --changelog

# Pasang hook git
commitkit --install-hooks

# Papar statistik commit bagi 30 hari lalu
commitkit --stats --days 30

# Sahkan mesej commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Pembangunan

### Bina

```bash
# Binaan nyahpepijat (Debug)
cargo build

# Binaan keluaran (Release)
cargo build --release
```

### Ujian

```bash
# Jalankan semua ujian
cargo test

# Jalankan ujian bersama output
cargo test -- --nocapture
```

### Lint

```bash
# Semak isu linting
cargo clippy --all-targets --all-features

# Gunakan pembetulan automatik
cargo clippy --fix --allow-dirty
```

### Pemformatan

```bash
# Semak pemformatan
cargo fmt -- --check

# Laksanakan pemformatan
cargo fmt
```

## Konfigurasi

CommitKit mencari fail konfigurasi bernama `.commitkit.toml` di lokasi berikut (mengikut urutan):

1. Direktori semasa
2. Direktori rumah pengguna

Jika tiada fail konfigurasi ditemui, fail lalai akan dijana secara automatik dalam direktori rumah anda.

Contoh konfigurasi:

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

## Lesen

Projek ini dilesenkan di bawah Lesen MIT — lihat fail [LICENSE](../../../LICENSE) untuk butiran lanjut.

## Menyumbang

Sumbangan amat dialu-alukan! Anda bebas membuka isu atau menghantar pull request.

1. Fork repositori ini
2. Cipta cabang ciri anda (`git checkout -b feature/amazing-feature`)
3. Commit perubahan anda menggunakan conventional commits
4. Tolak cabang tersebut (`git push origin feature/amazing-feature`)
5. Buka Pull Request
