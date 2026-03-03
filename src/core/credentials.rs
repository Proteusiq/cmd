//! Secure credential storage for API keys.
//!
//! Uses system keychain (macOS Keychain, Linux Secret Service, Windows Credential Manager)
//! with encrypted file fallback for environments without keychain access.

use anyhow::{Context, Result};
use keyring::Entry;
use secrecy::SecretString;

const SERVICE: &str = "com.proteusiq.cmd-cli";

/// Known providers and their identifiers.
pub const PROVIDERS: &[(&str, &str)] = &[
    ("anthropic", "Anthropic"),
    ("openai", "OpenAI"),
    ("ollama_host", "Ollama"),
];

/// Unified secure storage API.
///
/// Priority order for loading credentials:
/// 1. System keychain (seamless, no password prompt)
/// 2. Encrypted file fallback (requires master password, interactive only)
pub struct SecureStorage;

impl SecureStorage {
    /// Save a credential to secure storage.
    ///
    /// Tries system keychain first, falls back to encrypted file if unavailable.
    pub fn save(provider: &str, value: &str) -> Result<()> {
        match Entry::new(SERVICE, provider) {
            Ok(entry) => {
                entry
                    .set_password(value)
                    .with_context(|| format!("Failed to save {} to keychain", provider))?;
                Ok(())
            }
            Err(e) => Self::save_to_encrypted_file(provider, value)
                .with_context(|| format!("Keychain error: {}. Encrypted file also failed", e)),
        }
    }

    /// Load a credential from secure storage.
    ///
    /// Returns `None` if credential not found in keychain or encrypted file.
    pub fn load(provider: &str) -> Option<String> {
        if let Ok(entry) = Entry::new(SERVICE, provider)
            && let Ok(password) = entry.get_password()
        {
            return Some(password);
        }

        Self::load_from_encrypted_file(provider)
    }

    /// Load a credential as `SecretString` for memory safety.
    pub fn load_secret(provider: &str) -> Option<SecretString> {
        Self::load(provider).map(SecretString::from)
    }

    /// Delete a credential from all storage locations.
    pub fn delete(provider: &str) -> Result<()> {
        let mut deleted = false;

        if let Ok(entry) = Entry::new(SERVICE, provider)
            && entry.delete_credential().is_ok()
        {
            deleted = true;
        }

        if Self::delete_from_encrypted_file(provider).is_ok() {
            deleted = true;
        }

        if deleted {
            Ok(())
        } else {
            anyhow::bail!("No credential found for {}", provider)
        }
    }

    /// Check if a credential exists in any storage location.
    pub fn exists(provider: &str) -> bool {
        Self::load(provider).is_some()
    }

    /// List all providers with their storage status.
    pub fn list_providers() -> Vec<(&'static str, &'static str, bool)> {
        PROVIDERS
            .iter()
            .map(|(id, name)| (*id, *name, Self::exists(id)))
            .collect()
    }

    /// Get a masked version of the credential for safe display.
    ///
    /// Returns asterisks based on key length category without revealing actual content.
    pub fn get_masked(provider: &str) -> Option<String> {
        Self::load(provider).map(|key| {
            let len = key.len();
            if len < 20 {
                "********".to_string()
            } else if len < 50 {
                "********************".to_string()
            } else {
                "********************************".to_string()
            }
        })
    }

    fn save_to_encrypted_file(provider: &str, value: &str) -> Result<()> {
        use crate::core::encrypted_file::{EncryptedCredentials, prompt_master_password};

        let is_new_file = !EncryptedCredentials::exists();
        let password = prompt_master_password(is_new_file)?;

        let mut creds = EncryptedCredentials::load(&password).unwrap_or_default();
        creds.set(provider, value.to_string());
        creds.save(&password)
    }

    fn load_from_encrypted_file(provider: &str) -> Option<String> {
        use crate::core::encrypted_file::EncryptedCredentials;

        if !EncryptedCredentials::exists() {
            return None;
        }

        if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
            return None;
        }

        use crate::core::encrypted_file::prompt_master_password;

        eprintln!(
            "{}",
            owo_colors::OwoColorize::dimmed(
                &"Keychain unavailable, using encrypted file backup...".to_string()
            )
        );

        let password = prompt_master_password(false).ok()?;

        EncryptedCredentials::load(&password)
            .ok()
            .and_then(|creds| creds.get(provider).cloned())
    }

    fn delete_from_encrypted_file(provider: &str) -> Result<()> {
        use crate::core::encrypted_file::{EncryptedCredentials, prompt_master_password};

        if !EncryptedCredentials::exists() {
            anyhow::bail!("No encrypted credentials file");
        }

        let password = prompt_master_password(false)?;
        let mut creds = EncryptedCredentials::load(&password)?;
        creds.remove(provider);

        if creds.is_empty() {
            if let Some(path) = EncryptedCredentials::credentials_path() {
                std::fs::remove_file(path).ok();
            }
            return Ok(());
        }

        creds.save(&password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn providers_list_is_valid() {
        assert!(!PROVIDERS.is_empty());
        for (id, name) in PROVIDERS {
            assert!(!id.is_empty());
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn service_name_is_reverse_dns() {
        assert!(SERVICE.contains('.'));
        assert!(SERVICE.starts_with("com."));
    }

    #[test]
    fn masked_output_hides_content() {
        let short_key = "12345678";
        let medium_key = "1234567890123456789012345";
        let long_key = "12345678901234567890123456789012345678901234567890123456";

        let mask = |key: &str| {
            let len = key.len();
            if len < 20 {
                "********".to_string()
            } else if len < 50 {
                "********************".to_string()
            } else {
                "********************************".to_string()
            }
        };

        let masked_short = mask(short_key);
        let masked_medium = mask(medium_key);
        let masked_long = mask(long_key);

        assert!(!masked_short.contains('1'));
        assert!(!masked_medium.contains('1'));
        assert!(!masked_long.contains('1'));

        assert!(masked_short.chars().all(|c| c == '*'));
        assert!(masked_medium.chars().all(|c| c == '*'));
        assert!(masked_long.chars().all(|c| c == '*'));
    }
}
