// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

//! Internationalization (i18n) support for CommitKit.
//!
//! Provides compile-time checked translations for 20 locales with POSIX
//! environment detection (`LC_ALL` -> `LC_MESSAGES` -> `LANG`) and optional
//! explicit override via `--lang`.

pub mod catalog;

pub use catalog::Catalog;

/// Supported display locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Locale {
    #[default]
    En,
    De,
    Es,
    Fr,
    It,
    PtBr,
    Nl,
    Ru,
    Uk,
    Ja,
    Ko,
    ZhCn,
    ZhTw,
    Ar,
    Hi,
    Pl,
    Tr,
    Sv,
    Vi,
    Id,
}

impl Locale {
    /// All 20 supported locales.
    pub const ALL: [Locale; 20] = [
        Locale::En,
        Locale::De,
        Locale::Es,
        Locale::Fr,
        Locale::It,
        Locale::PtBr,
        Locale::Nl,
        Locale::Ru,
        Locale::Uk,
        Locale::Ja,
        Locale::Ko,
        Locale::ZhCn,
        Locale::ZhTw,
        Locale::Ar,
        Locale::Hi,
        Locale::Pl,
        Locale::Tr,
        Locale::Sv,
        Locale::Vi,
        Locale::Id,
    ];

    /// Canonical BCP 47 language tag for this locale.
    pub const fn code(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::De => "de",
            Locale::Es => "es",
            Locale::Fr => "fr",
            Locale::It => "it",
            Locale::PtBr => "pt-BR",
            Locale::Nl => "nl",
            Locale::Ru => "ru",
            Locale::Uk => "uk",
            Locale::Ja => "ja",
            Locale::Ko => "ko",
            Locale::ZhCn => "zh-CN",
            Locale::ZhTw => "zh-TW",
            Locale::Ar => "ar",
            Locale::Hi => "hi",
            Locale::Pl => "pl",
            Locale::Tr => "tr",
            Locale::Sv => "sv",
            Locale::Vi => "vi",
            Locale::Id => "id",
        }
    }

    /// Parse a BCP 47 or POSIX locale tag (e.g. `de_DE.UTF-8`, `pt-BR`, `zh_Hans_CN`, `C`).
    pub fn from_tag(tag: &str) -> Option<Locale> {
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            return None;
        }

        // Strip POSIX encoding (`.UTF-8`) and modifier (`@euro`).
        let base = trimmed
            .split('@')
            .next()
            .unwrap_or(trimmed)
            .split('.')
            .next()
            .unwrap_or(trimmed);

        let normalized = base.replace('_', "-").to_ascii_lowercase();

        if normalized == "c" || normalized == "posix" {
            return Some(Locale::En);
        }

        match normalized.as_str() {
            "en" => return Some(Locale::En),
            "de" => return Some(Locale::De),
            "es" => return Some(Locale::Es),
            "fr" => return Some(Locale::Fr),
            "it" => return Some(Locale::It),
            "pt" | "pt-br" => return Some(Locale::PtBr),
            "nl" => return Some(Locale::Nl),
            "ru" => return Some(Locale::Ru),
            "uk" => return Some(Locale::Uk),
            "ja" => return Some(Locale::Ja),
            "ko" => return Some(Locale::Ko),
            "zh" | "zh-cn" | "zh-sg" | "zh-hans" | "zh-hans-cn" | "zh-hans-sg" => {
                return Some(Locale::ZhCn);
            }
            "zh-tw" | "zh-hk" | "zh-mo" | "zh-hant" | "zh-hant-tw" | "zh-hant-hk" => {
                return Some(Locale::ZhTw);
            }
            "ar" => return Some(Locale::Ar),
            "hi" => return Some(Locale::Hi),
            "pl" => return Some(Locale::Pl),
            "tr" => return Some(Locale::Tr),
            "sv" => return Some(Locale::Sv),
            "vi" => return Some(Locale::Vi),
            "id" | "in" => return Some(Locale::Id),
            _ => {}
        }

        let primary = normalized.split('-').next().unwrap_or("");
        match primary {
            "en" => Some(Locale::En),
            "de" => Some(Locale::De),
            "es" => Some(Locale::Es),
            "fr" => Some(Locale::Fr),
            "it" => Some(Locale::It),
            "pt" => Some(Locale::PtBr),
            "nl" => Some(Locale::Nl),
            "ru" => Some(Locale::Ru),
            "uk" => Some(Locale::Uk),
            "ja" => Some(Locale::Ja),
            "ko" => Some(Locale::Ko),
            "zh" => {
                if normalized.contains("hant")
                    || normalized.contains("-tw")
                    || normalized.contains("-hk")
                    || normalized.contains("-mo")
                {
                    Some(Locale::ZhTw)
                } else {
                    Some(Locale::ZhCn)
                }
            }
            "ar" => Some(Locale::Ar),
            "hi" => Some(Locale::Hi),
            "pl" => Some(Locale::Pl),
            "tr" => Some(Locale::Tr),
            "sv" => Some(Locale::Sv),
            "vi" => Some(Locale::Vi),
            "id" | "in" => Some(Locale::Id),
            _ => None,
        }
    }

    /// Detect locale from POSIX environment variables (`LC_ALL` -> `LC_MESSAGES` -> `LANG`).
    pub fn detect() -> Locale {
        for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            if let Ok(value) = std::env::var(var) {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if let Some(locale) = Self::from_tag(trimmed) {
                    return locale;
                }
                return Locale::En;
            }
        }
        Locale::En
    }

    /// Resolve locale from an optional explicit tag, falling back to environment detection.
    pub fn resolve(explicit: Option<&str>) -> Locale {
        if let Some(tag) = explicit {
            if let Some(locale) = Self::from_tag(tag) {
                return locale;
            }
            return Locale::En;
        }
        Self::detect()
    }

    /// Return the static message catalog for this locale.
    pub const fn catalog(self) -> &'static Catalog {
        catalog::for_locale(self)
    }
}

/// Extract `--lang <value>` or `--lang=<value>` from raw CLI arguments so `--help`
/// output can be localized before full argument parsing runs.
pub fn extract_cli_lang(args: &[String]) -> Option<String> {
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        if arg == "--" {
            break;
        }
        if arg == "--lang" {
            return iter.next().cloned();
        }
        if let Some(val) = arg.strip_prefix("--lang=") {
            return Some(val.to_string());
        }
    }
    None
}

/// Replace a single `{name}` placeholder in a catalog template.
pub fn fmt1(template: &str, key: &str, value: &str) -> String {
    template.replace(&format!("{{{key}}}"), value)
}

/// Replace two `{name}` placeholders in a catalog template.
pub fn fmt2(template: &str, key1: &str, val1: &str, key2: &str, val2: &str) -> String {
    template
        .replace(&format!("{{{key1}}}"), val1)
        .replace(&format!("{{{key2}}}"), val2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_placeholders(text: &str) -> Vec<String> {
        let mut placeholders = Vec::new();
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'{' {
                if let Some(end) = text[i + 1..].find('}') {
                    let token = &text[i + 1..i + 1 + end];
                    if !token.is_empty()
                        && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        placeholders.push(format!("{{{token}}}"));
                        i += end + 2;
                        continue;
                    }
                }
            }
            i += 1;
        }
        placeholders.sort();
        placeholders
    }

    #[test]
    fn test_all_20_locales_parse() {
        assert_eq!(Locale::ALL.len(), 20);
        for locale in Locale::ALL {
            assert_eq!(Locale::from_tag(locale.code()), Some(locale));
        }
        assert_eq!(Locale::from_tag("C"), Some(Locale::En));
        assert_eq!(Locale::from_tag("POSIX"), Some(Locale::En));
        assert_eq!(Locale::from_tag("en_US.UTF-8"), Some(Locale::En));
        assert_eq!(Locale::from_tag("de_DE.UTF-8@euro"), Some(Locale::De));
        assert_eq!(Locale::from_tag("pt_BR.UTF-8"), Some(Locale::PtBr));
        assert_eq!(Locale::from_tag("zh_Hans_CN.UTF-8"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh_TW.UTF-8"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-Hant"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag(""), None);
        assert_eq!(Locale::from_tag("xx-UNKNOWN"), None);
    }

    #[test]
    fn test_all_catalog_entries_non_empty_and_placeholder_parity() {
        let en = Locale::En.catalog().entries();
        assert_eq!(en.len(), 67);

        for locale in Locale::ALL {
            let entries = locale.catalog().entries();
            assert_eq!(entries.len(), en.len());
            for ((en_key, en_val), (loc_key, loc_val)) in en.iter().zip(entries.iter()) {
                assert_eq!(en_key, loc_key);
                assert!(
                    !loc_val.trim().is_empty(),
                    "Empty translation for {loc_key} in {}",
                    locale.code()
                );
                assert_eq!(
                    extract_placeholders(loc_val),
                    extract_placeholders(en_val),
                    "Placeholder mismatch for {loc_key} in {}: got {loc_val:?}",
                    locale.code()
                );
            }
        }
    }

    #[test]
    fn test_default_english_strings_backward_compatible() {
        let en = Locale::En.catalog();
        assert_eq!(en.cli_about, "CLI tool for creating conventional commits");
        assert_eq!(en.prompt_select_type, "Select commit type");
        assert_eq!(en.msg_commit_valid, "Commit message is valid.");
        assert_eq!(en.err_not_git_repo, "Not in a git repository");
    }

    #[test]
    fn test_extract_cli_lang() {
        let args = vec![
            "commitkit".to_string(),
            "--lang".to_string(),
            "ja".to_string(),
            "--help".to_string(),
        ];
        assert_eq!(extract_cli_lang(&args), Some("ja".to_string()));

        let args_eq = vec!["commitkit".to_string(), "--lang=pt-BR".to_string()];
        assert_eq!(extract_cli_lang(&args_eq), Some("pt-BR".to_string()));

        let args_none = vec!["commitkit".to_string(), "--dry-run".to_string()];
        assert_eq!(extract_cli_lang(&args_none), None);
    }
}
