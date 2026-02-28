use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = "cmd";
const CONFIG_FILE: &str = "settings.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub enable_execution: bool,
    #[serde(default)]
    pub skip_confirmation: bool,
}

impl Settings {
    /// Load settings from config file, or return defaults if not found
    pub fn load() -> Self {
        Self::config_path()
            .and_then(|path| fs::read_to_string(&path).ok())
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default()
    }

    /// Save settings to config file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path().context("Could not determine config directory")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize settings")?;
        fs::write(&path, content).context("Failed to write config file")?;

        Ok(())
    }

    /// Get the config file path
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join(CONFIG_DIR).join(CONFIG_FILE))
    }

    /// Update enable_execution setting
    pub fn set_enable_execution(&mut self, value: bool) {
        self.enable_execution = value;
    }

    /// Update skip_confirmation setting
    pub fn set_skip_confirmation(&mut self, value: bool) {
        self.skip_confirmation = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_are_safe() {
        let settings = Settings::default();
        assert!(!settings.enable_execution);
        assert!(!settings.skip_confirmation);
    }

    #[test]
    fn deserialize_partial_config() {
        let toml = r#"
            enable_execution = true
        "#;
        let settings: Settings = toml::from_str(toml).unwrap();
        assert!(settings.enable_execution);
        assert!(!settings.skip_confirmation);
    }
}
