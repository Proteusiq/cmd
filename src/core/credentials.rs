//! Encrypted credential storage for API keys
//!
//! Uses AES-256-GCM encryption with Argon2id key derivation.
//! Credentials are stored in ~/.config/cmd/credentials.enc

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{bail, Context, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = "cmd";
const CREDENTIALS_FILE: &str = "credentials.enc";
const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 22; // Standard Argon2 salt size

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Credentials {
    pub anthropic_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub ollama_host: Option<String>,
    pub custom_endpoint: Option<String>,
}

/// Encrypted credential file format
#[derive(Serialize, Deserialize)]
struct EncryptedFile {
    salt: String,       // Base64 encoded salt for key derivation
    nonce: String,      // Base64 encoded nonce for AES-GCM
    ciphertext: String, // Base64 encoded encrypted credentials
}

impl Credentials {
    /// Load credentials from encrypted file using master password
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
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|_| anyhow::anyhow!("Failed to create cipher"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| anyhow::anyhow!("Decryption failed - wrong password?"))?;

        let credentials: Credentials =
            serde_json::from_slice(&plaintext).context("Failed to parse decrypted credentials")?;

        Ok(credentials)
    }

    /// Save credentials to encrypted file using master password
    pub fn save(&self, master_password: &str) -> Result<()> {
        let path = Self::credentials_path().context("Could not determine config directory")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;

            // Set directory permissions to 700 (owner only)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let perms = std::fs::Permissions::from_mode(0o700);
                fs::set_permissions(parent, perms).ok();
            }
        }

        // Generate random salt and nonce
        let mut salt = vec![0u8; SALT_SIZE];
        let mut nonce_bytes = vec![0u8; NONCE_SIZE];
        rand::rng().fill_bytes(&mut salt);
        rand::rng().fill_bytes(&mut nonce_bytes);

        let key = derive_key(master_password, &salt)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
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

        // Set file permissions to 600 (owner read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            fs::set_permissions(&path, perms).ok();
        }

        Ok(())
    }

    /// Check if credentials file exists
    pub fn exists() -> bool {
        Self::credentials_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }

    /// Get the credentials file path
    pub fn credentials_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join(CONFIG_DIR).join(CREDENTIALS_FILE))
    }

    /// Set a credential by provider name
    pub fn set(&mut self, provider: &str, value: String) {
        match provider {
            "anthropic" => self.anthropic_api_key = Some(value),
            "openai" => self.openai_api_key = Some(value),
            "ollama" => self.ollama_host = Some(value),
            _ => {}
        }
    }

    /// Check if any credentials are stored
    pub fn is_empty(&self) -> bool {
        self.anthropic_api_key.is_none()
            && self.openai_api_key.is_none()
            && self.ollama_host.is_none()
    }
}

/// Derive a 256-bit key from password using Argon2id
fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>> {
    // Create a SaltString from raw bytes
    let salt_string =
        SaltString::encode_b64(salt).map_err(|e| anyhow::anyhow!("Invalid salt: {}", e))?;

    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| anyhow::anyhow!("Key derivation failed: {}", e))?;

    // Extract the hash output (32 bytes for AES-256)
    let hash_output = hash.hash.context("No hash output")?;
    let key_bytes = hash_output.as_bytes();

    // Ensure we have exactly 32 bytes for AES-256
    if key_bytes.len() < 32 {
        bail!("Derived key too short");
    }

    Ok(key_bytes[..32].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_encryption() {
        let mut creds = Credentials::default();
        creds.anthropic_api_key = Some("sk-ant-test-key".to_string());
        creds.openai_api_key = Some("sk-openai-test".to_string());

        let password = "test-master-password";

        // Serialize and encrypt
        let mut salt = vec![0u8; SALT_SIZE];
        let mut nonce_bytes = vec![0u8; NONCE_SIZE];
        rand::rng().fill_bytes(&mut salt);
        rand::rng().fill_bytes(&mut nonce_bytes);

        let key = derive_key(password, &salt).unwrap();
        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = serde_json::to_vec(&creds).unwrap();
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();

        // Decrypt and deserialize
        let decrypted = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
        let loaded: Credentials = serde_json::from_slice(&decrypted).unwrap();

        assert_eq!(loaded.anthropic_api_key, creds.anthropic_api_key);
        assert_eq!(loaded.openai_api_key, creds.openai_api_key);
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
        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = b"secret data";
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();

        // Try to decrypt with wrong password
        let wrong_key = derive_key(wrong_password, &salt).unwrap();
        let wrong_cipher = Aes256Gcm::new_from_slice(&wrong_key).unwrap();

        let result = wrong_cipher.decrypt(nonce, ciphertext.as_ref());
        assert!(result.is_err());
    }
}
