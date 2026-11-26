[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit, geliştiricilerin [Conventional Commits](https://www.conventionalcommits.org/) belirtimine uygun, düzenli ve tutarlı commit mesajları oluşturmasına yardımcı olan bir komut satırı aracıdır. Tür, isteğe bağlı kapsam, konu, gövde ve altbilgi içeren bir commit mesajı oluştururken kullanıcıya adım adım rehberlik eden etkileşimli bir arayüz sunar.

## Özellikler

- **Temel İşlevler**
  - Commit türleri ve kapsamlarının etkileşimli seçimi
  - `.commitkit.toml` ile özelleştirilebilir yapılandırma
  - Maksimum konu uzunluğu denetimi
  - Çok satırlı commit gövdesi desteği
  - Girdilerin otomatik doğrulanması
  - Commit'lere isteğe bağlı `Signed-off-by` satırı ekleme
  - Çapraz platform uyumluluğu

- **Gelişmiş Özellikler**
  - Commit mesajlarında emoji desteği (✨, 🐛 vb.)
  - Yer tutucu değiştirmeli commit şablonları
  - Otomatik değişiklik günlüğü oluşturma ve güncelleme
  - Commit mesajlarını doğrulama ve biçimlendirme için Git kancaları
  - Commit istatistikleri ve analizi
  - Şablon değerleri için etkileşimli istemler

## Ön Koşullar

- **Rust**: Sürüm 1.88.0 veya üzeri (MSRV)
- **Cargo**: Rust kurulumuna dahildir

## Kurulum

### Kaynak koddan

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Kullanım

Git deponuza gidin ve şu komutu çalıştırın:

```shell
commitkit
```

### Komut satırı seçenekleri

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit komutunu atla ve mesajı standart çıktıya yazdır
    -v, --verbose           Ayrıntılı çıktıyı göster
    -c, --config            Yapılandırma dosyası yolu (varsayılan: geçerli veya ev dizinindeki .commitkit.toml)
        --emoji             Commit mesajlarında emoji kullanımını aç/kapat
        --template          Belirli bir commit şablonu kullan
        --changelog         Commit işleminden sonra değişiklik günlüğünü güncelle
        --install-hooks     Git kancalarını kur
        --stats             Commit istatistiklerini göster
        --days              İstatistikler için analiz edilecek gün sayısı (varsayılan: tümü)
        --validate          Bir commit mesajı dosyasını doğrula
        --prepare-msg       Commit mesajını hazırla (git kancası tarafından kullanılır)
        --lang <LANG>       Görüntüleme dili (örn. en, de, ja, zh-CN, pt-BR)
    -h, --help              Yardım bilgilerini yazdır
    -V, --version           Sürüm bilgilerini yazdır
```

### Desteklenen Diller

CommitKit, görüntüleme dilini `LC_ALL`, `LC_MESSAGES` ve `LANG` ortam değişkenlerinden algılar veya `--lang <LANG>` seçeneğini kabul eder. Sağdan sola yazılan 6 dil (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, `ur`) dahil 45 dili destekler. Tam liste için [İngilizce README](../../../README.md) dosyasına bakın.

### Etkileşimli süreç

Araç, kurallı bir commit mesajı oluşturmanız için size rehberlik eder:

1. Bir commit türü seçin (`feat`, `fix`, `docs` vb.)
2. Bir kapsam seçin (isteğe bağlı)
3. Commit konusunu girin
4. Commit gövdesini girin (isteğe bağlı)
5. Commit altbilgisini girin (isteğe bağlı)

Bu adımlar tamamlandıktan sonra CommitKit oluşturduğunuz mesajla `git commit` komutunu çalıştırır.

### Örnekler

```shell
# Temel kullanım
commitkit

# Deneme modu - commit oluşturmadan sonuç mesajını göster
commitkit --dry-run

# Özel yapılandırma dosyası kullan
commitkit --config /path/to/custom-config.toml

# Ayrıntılı çıktı
commitkit --verbose

# Commit mesajlarında emoji kullan
commitkit --emoji

# Commit için şablon kullan
commitkit --template feature

# CHANGELOG.md dosyasını otomatik güncelle
commitkit --changelog

# Git kancalarını kur
commitkit --install-hooks

# Son 30 günün commit istatistiklerini göster
commitkit --stats --days 30

# Bir commit mesajını doğrula
commitkit --validate .git/COMMIT_EDITMSG
```

## Geliştirme

### Derleme

```bash
# Hata ayıklama derlemesi
cargo build

# Sürüm derlemesi
cargo build --release
```

### Test

```bash
# Tüm testleri çalıştır
cargo test

# Çıktılarla birlikte testleri çalıştır
cargo test -- --nocapture
```

### Lint

```bash
# Lint uyarılarını denetle
cargo clippy --all-targets --all-features

# Otomatik düzeltmeleri uygula
cargo clippy --fix --allow-dirty
```

### Biçimlendirme

```bash
# Biçimlendirmeyi denetle
cargo fmt -- --check

# Biçimlendirmeyi uygula
cargo fmt
```

## Yapılandırma

CommitKit, `.commitkit.toml` adlı yapılandırma dosyasını şu konumlarda (sırasıyla) arar:

1. Geçerli dizin
2. Kullanıcının ev dizini

Yapılandırma dosyası bulunamazsa ev dizininizde varsayılan bir dosya oluşturulur.

Örnek yapılandırma:

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

## Lisans

Bu proje MIT Lisansı ile lisanslanmıştır — ayrıntılar için [LICENSE](../../../LICENSE) dosyasına bakın.

## Katkıda Bulunma

Katkılarınız memnuniyetle karşılanır! Sorun (issue) açabilir veya pull request gönderebilirsiniz.

1. Depoyu çatallayın (fork)
2. Özellik dalınızı oluşturun (`git checkout -b feature/amazing-feature`)
3. Değişikliklerinizi kurallı commit mesajlarıyla kaydedin
4. Dalı gönderin (`git push origin feature/amazing-feature`)
5. Bir Pull Request açın
