[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit adalah alat baris perintah yang membantu pengembang membuat pesan commit yang terstruktur dan konsisten sesuai spesifikasi [Conventional Commits](https://www.conventionalcommits.org/). Alat ini menyediakan antarmuka interaktif untuk memandu pengguna menyusun pesan commit dengan tipe, lingkup opsional, subjek, isi, dan footer.

## Fitur

- **Fungsionalitas Inti**
  - Pemilihan tipe commit dan lingkup secara interaktif
  - Konfigurasi yang dapat disesuaikan melalui `.commitkit.toml`
  - Penegakan panjang maksimum subjek
  - Dukungan isi commit multibaris
  - Validasi input otomatis
  - Opsi commit bertanda tangan (`Signed-off-by`)
  - Kompatibilitas lintas platform

- **Fitur Tambahan**
  - Dukungan emoji dalam pesan commit (✨, 🐛, dll.)
  - Templat commit dengan penggantian placeholder
  - Pembuatan dan pembaruan changelog otomatis
  - Hook Git untuk validasi dan pemformatan pesan commit
  - Statistik dan analisis commit
  - Prompt interaktif untuk nilai templat

## Prasyarat

- **Rust**: Versi 1.88.0 atau lebih tinggi (MSRV)
- **Cargo**: Termasuk dalam instalasi Rust

## Instalasi

### Dari kode sumber

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Penggunaan

Arahkan ke repositori Git Anda dan jalankan:

```shell
commitkit
```

### Opsi baris perintah

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Lewati git commit dan cetak pesan ke stdout
    -v, --verbose           Tampilkan output lengkap
    -c, --config            Jalur ke file konfigurasi (bawaan: .commitkit.toml di direktori saat ini atau home)
        --emoji             Aktifkan/nonaktifkan emoji dalam pesan commit
        --template          Gunakan templat commit tertentu
        --changelog         Perbarui changelog setelah melakukan commit
        --install-hooks     Pasang hook git
        --stats             Tampilkan statistik commit
        --days              Jumlah hari yang dianalisis untuk statistik (bawaan: semua)
        --validate          Validasi file pesan commit
        --prepare-msg       Siapkan pesan commit (digunakan oleh hook git)
        --lang <LANG>       Bahasa tampilan (mis. en, de, ja, zh-CN, pt-BR)
    -h, --help              Cetak informasi bantuan
    -V, --version           Cetak informasi versi
```

### Bahasa yang Didukung

CommitKit mendeteksi bahasa tampilan dari `LC_ALL`, `LC_MESSAGES`, dan `LANG`, atau menerima `--lang <LANG>`. Alat ini mendukung 45 bahasa, termasuk 6 bahasa kanan-ke-kiri (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, dan `ur`). Lihat tabel lengkapnya di [README bahasa Inggris](../../../README.md).

### Proses interaktif

Alat ini akan memandu Anda dalam membuat pesan commit konvensional:

1. Pilih tipe commit (`feat`, `fix`, `docs`, dll.)
2. Pilih lingkup (opsional)
3. Masukkan subjek commit
4. Masukkan isi commit (opsional)
5. Masukkan footer commit (opsional)

Setelah langkah-langkah ini selesai, CommitKit akan menjalankan `git commit` dengan pesan yang telah disusun.

### Contoh

```shell
# Penggunaan dasar
commitkit

# Uji coba - tampilkan pesan commit tanpa melakukan commit
commitkit --dry-run

# Gunakan file konfigurasi khusus
commitkit --config /path/to/custom-config.toml

# Output lengkap
commitkit --verbose

# Gunakan emoji dalam pesan commit
commitkit --emoji

# Gunakan templat untuk commit Anda
commitkit --template feature

# Perbarui CHANGELOG.md secara otomatis
commitkit --changelog

# Pasang hook git
commitkit --install-hooks

# Tampilkan statistik commit untuk 30 hari terakhir
commitkit --stats --days 30

# Validasi pesan commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Pengembangan

### Bangun

```bash
# Build debug
cargo build

# Build rilis
cargo build --release
```

### Pengujian

```bash
# Jalankan semua pengujian
cargo test

# Jalankan pengujian dengan output
cargo test -- --nocapture
```

### Lint

```bash
# Periksa masalah linting
cargo clippy --all-targets --all-features

# Terapkan perbaikan otomatis
cargo clippy --fix --allow-dirty
```

### Pemformatan

```bash
# Periksa pemformatan
cargo fmt -- --check

# Terapkan pemformatan
cargo fmt
```

## Konfigurasi

CommitKit mencari file konfigurasi bernama `.commitkit.toml` di lokasi berikut (secara berurutan):

1. Direktori saat ini
2. Direktori home pengguna

Jika file konfigurasi tidak ditemukan, file bawaan akan dibuat di direktori home Anda.

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

## Lisensi

Proyek ini dilisensikan di bawah Lisensi MIT — lihat file [LICENSE](../../../LICENSE) untuk detailnya.

## Berkontribusi

Kontribusi sangat diterima! Silakan buka issue atau kirim pull request.

1. Fork repositori ini
2. Buat cabang fitur Anda (`git checkout -b feature/amazing-feature`)
3. Commit perubahan Anda menggunakan conventional commits
4. Push ke cabang tersebut (`git push origin feature/amazing-feature`)
5. Buka Pull Request
