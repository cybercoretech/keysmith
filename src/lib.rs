//! Reusable, local-first Keysmith generation API.
//!
//! Applications should call this crate (or implement the documented
//! `passphrase/v1` wire contract) so secrets are generated on the user's
//! device and never need to be sent to a server for generation.

pub mod passphrase;
pub mod wordlist;

/// Stable options accepted by Keysmith's passphrase generator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PassphraseOptions {
    pub words: usize,
    pub separator: String,
    pub capitalize: bool,
}

impl Default for PassphraseOptions {
    fn default() -> Self {
        Self {
            words: 6,
            separator: "-".into(),
            capitalize: false,
        }
    }
}

/// Generate a passphrase using the full embedded EFF large wordlist and the
/// operating system CSPRNG. The returned value is secret material: callers
/// must avoid logging it or putting it in URLs/command arguments.
pub fn generate_passphrase(options: &PassphraseOptions) -> Result<String, &'static str> {
    if !(3..=12).contains(&options.words) {
        return Err("word count must be between 3 and 12");
    }
    if options.separator.len() > 8 || options.separator.chars().any(char::is_control) {
        return Err("separator must be at most 8 printable bytes");
    }
    passphrase::generate(
        options.words,
        &options.separator,
        options.capitalize,
        &wordlist::words(),
    )
    .ok_or("the wordlist is unavailable")
}

/// Versioned integration contract used by the browser and Android clients.
/// The `words` array is intentionally not returned: callers ship the signed
/// EFF list as an asset and use their platform CSPRNG locally.
pub const API_VERSION: &str = "passphrase/v1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_valid_default_passphrase_from_eff_list() {
        let phrase = generate_passphrase(&PassphraseOptions::default()).unwrap();
        let words = phrase.split('-').collect::<Vec<_>>();
        assert_eq!(words.len(), 6);
        assert!(words.iter().all(|word| wordlist::words().contains(word)));
    }

    #[test]
    fn rejects_unsafe_or_out_of_range_options() {
        assert!(generate_passphrase(&PassphraseOptions {
            words: 2,
            ..Default::default()
        })
        .is_err());
        assert!(generate_passphrase(&PassphraseOptions {
            words: 13,
            ..Default::default()
        })
        .is_err());
        assert!(generate_passphrase(&PassphraseOptions {
            separator: "\n".into(),
            ..Default::default()
        })
        .is_err());
    }
}
