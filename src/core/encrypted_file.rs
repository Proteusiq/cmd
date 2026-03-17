//! Encrypted credential file storage (fallback when keychain is unavailable).
//!
//! Uses AES-256-GCM encryption with Argon2id key derivation.
//! Credentials are stored in `~/.config/cmd/credentials.enc`.

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use anyhow::{Context, Result, bail};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version, password_hash::SaltString};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use rand::RngCore;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = "cmd";
const CREDENTIALS_FILE: &str = "credentials.enc";
const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 16;

/// Credentials stored in the encrypted file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    #[serde(flatten)]
    credentials: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct EncryptedFile {
    salt: String,
    nonce: String,
    ciphertext: String,
}

impl EncryptedCredentials {
    /// Load credentials from encrypted file using master password.
    pub fn load(master_password: &str) -> Result<Self> {
        let path = Self::credentials_path().context("Could not determine config directory")?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).context("Failed to read credentials file")?;
        let encrypted: EncryptedFile =
            serde_json::from_str(&content).context("Failed to parse credentials file")?;

        let salt = BASE64
            .decode(&encrypted.salt)
            .context("Invalid salt encoding")?;
        let nonce_bytes = BASE64
            .decode(&encrypted.nonce)
            .context("Invalid nonce encoding")?;
        let ciphertext = BASE64
            .decode(&encrypted.ciphertext)
            .context("Invalid ciphertext encoding")?;

        let key = derive_key(master_password, &salt)?;
        let cipher = Aes256Gcm::new_from_slice(key.expose_secret())
            .map_err(|_| anyhow::anyhow!("Failed to create cipher"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| anyhow::anyhow!("Decryption failed - wrong password?"))?;

        let credentials: EncryptedCredentials =
            serde_json::from_slice(&plaintext).context("Failed to parse decrypted credentials")?;

        Ok(credentials)
    }

    /// Save credentials to encrypted file using master password.
    pub fn save(&self, master_password: &str) -> Result<()> {
        let path = Self::credentials_path().context("Could not determine config directory")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let perms = std::fs::Permissions::from_mode(0o700);
                if let Err(e) = fs::set_permissions(parent, perms) {
                    eprintln!(
                        "Warning: Could not set permissions on {}: {}",
                        parent.display(),
                        e
                    );
                }
            }
        }

        let mut salt = vec![0u8; SALT_SIZE];
        let mut nonce_bytes = vec![0u8; NONCE_SIZE];
        rand::rng().fill_bytes(&mut salt);
        rand::rng().fill_bytes(&mut nonce_bytes);

        let key = derive_key(master_password, &salt)?;
        let cipher = Aes256Gcm::new_from_slice(key.expose_secret())
            .map_err(|_| anyhow::anyhow!("Failed to create cipher"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = serde_json::to_vec(self).context("Failed to serialize credentials")?;

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|_| anyhow::anyhow!("Encryption failed"))?;

        let encrypted = EncryptedFile {
            salt: BASE64.encode(&salt),
            nonce: BASE64.encode(&nonce_bytes),
            ciphertext: BASE64.encode(&ciphertext),
        };

        let content = serde_json::to_string_pretty(&encrypted)
            .context("Failed to serialize encrypted file")?;

        fs::write(&path, content).context("Failed to write credentials file")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            if let Err(e) = fs::set_permissions(&path, perms) {
                eprintln!(
                    "Warning: Could not set permissions on {}: {}",
                    path.display(),
                    e
                );
            }
        }

        Ok(())
    }

    /// Check if credentials file exists.
    pub fn exists() -> bool {
        Self::credentials_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }

    /// Get the credentials file path.
    pub fn credentials_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join(CONFIG_DIR).join(CREDENTIALS_FILE))
    }

    /// Set a credential by provider name.
    pub fn set(&mut self, provider: &str, value: String) {
        self.credentials.insert(provider.to_string(), value);
    }

    /// Get a credential by provider name.
    pub fn get(&self, provider: &str) -> Option<&String> {
        self.credentials.get(provider)
    }

    /// Remove a credential by provider name.
    pub fn remove(&mut self, provider: &str) {
        self.credentials.remove(provider);
    }

    /// Check if any credentials are stored.
    pub fn is_empty(&self) -> bool {
        self.credentials.is_empty()
    }
}

/// Secret key wrapper that zeros memory on drop.
pub struct SecretKey(Vec<u8>);

impl Drop for SecretKey {
    fn drop(&mut self) {
        for byte in self.0.iter_mut() {
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

impl SecretKey {
    fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl ExposeSecret<[u8]> for SecretKey {
    fn expose_secret(&self) -> &[u8] {
        &self.0
    }
}

/// Derive a 256-bit key from password using Argon2id.
///
/// Uses OWASP-recommended parameters for password hashing:
/// - Memory: 64 MiB (65536 KiB)
/// - Iterations: 3
/// - Parallelism: 4
/// - Output length: 32 bytes (256 bits)
fn derive_key(password: &str, salt: &[u8]) -> Result<SecretKey> {
    let salt_string =
        SaltString::encode_b64(salt).map_err(|e| anyhow::anyhow!("Invalid salt: {}", e))?;

    let params = Params::new(
        65536,    // 64 MiB memory
        3,        // 3 iterations
        4,        // 4 parallel lanes
        Some(32), // 32-byte output
    )
    .map_err(|e| anyhow::anyhow!("Invalid Argon2 parameters: {}", e))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| anyhow::anyhow!("Key derivation failed: {}", e))?;

    let hash_output = hash.hash.context("No hash output")?;
    let key_bytes = hash_output.as_bytes();

    if key_bytes.len() < 32 {
        bail!("Derived key too short");
    }

    Ok(SecretKey::new(key_bytes[..32].to_vec()))
}

/// Prompt user for master password.
pub fn prompt_master_password(confirm: bool) -> Result<String> {
    use dialoguer::Password;

    let password: String = Password::new()
        .with_prompt("Master password")
        .interact()
        .context("Failed to read password")?;

    if password.is_empty() {
        bail!("Password cannot be empty");
    }

    if password.len() < 8 {
        bail!("Password must be at least 8 characters");
    }

    if confirm {
        let confirmation: String = Password::new()
            .with_prompt("Confirm password")
            .interact()
            .context("Failed to read password confirmation")?;

        if !constant_time_eq(password.as_bytes(), confirmation.as_bytes()) {
            bail!("Passwords do not match");
        }
    }

    Ok(password)
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_encryption() {
        let mut creds = EncryptedCredentials::default();
        creds.set("anthropic", "sk-ant-test-key".to_string());
        creds.set("openai", "sk-openai-test".to_string());

        let password = "test-master-password";

        let mut salt = vec![0u8; SALT_SIZE];
        let mut nonce_bytes = vec![0u8; NONCE_SIZE];
        rand::rng().fill_bytes(&mut salt);
        rand::rng().fill_bytes(&mut nonce_bytes);

        let key = derive_key(password, &salt).unwrap();
        let cipher = Aes256Gcm::new_from_slice(key.expose_secret()).unwrap();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = serde_json::to_vec(&creds).unwrap();
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();

        let decrypted = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
        let loaded: EncryptedCredentials = serde_json::from_slice(&decrypted).unwrap();

        assert_eq!(loaded.get("anthropic"), creds.get("anthropic"));
        assert_eq!(loaded.get("openai"), creds.get("openai"));
    }

    #[test]
    fn wrong_password_fails() {
        let password = "correct-password";
        let wrong_password = "wrong-password";

        let mut salt = vec![0u8; SALT_SIZE];
        let mut nonce_bytes = vec![0u8; NONCE_SIZE];
        rand::rng().fill_bytes(&mut salt);
        rand::rng().fill_bytes(&mut nonce_bytes);

        let key = derive_key(password, &salt).unwrap();
        let cipher = Aes256Gcm::new_from_slice(key.expose_secret()).unwrap();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = b"secret data";
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();

        let wrong_key = derive_key(wrong_password, &salt).unwrap();
        let wrong_cipher = Aes256Gcm::new_from_slice(wrong_key.expose_secret()).unwrap();

        let result = wrong_cipher.decrypt(nonce, ciphertext.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn constant_time_eq_works() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
        assert!(!constant_time_eq(b"", b"a"));
        assert!(constant_time_eq(b"", b""));
    }
}
