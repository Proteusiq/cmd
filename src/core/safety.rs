//! Destructive command detection for safety checks
//!
//! Detects potentially dangerous commands that could cause data loss,
//! system damage, or security issues.

use std::collections::HashSet;

/// Result of analyzing a command for safety
#[derive(Debug, Clone)]
pub struct SafetyCheck {
    pub is_destructive: bool,
    pub reasons: Vec<String>,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Safe,
    Warning,
    Dangerous,
    Critical,
}

/// Destructive command patterns and their severity
const CRITICAL_COMMANDS: &[(&str, &str)] = &[
    ("rm -rf /", "Removes entire filesystem"),
    ("rm -rf /*", "Removes entire filesystem"),
    ("rm -rf ~", "Removes entire home directory"),
    ("rm -rf ~/*", "Removes entire home directory"),
    (":(){:|:&};:", "Fork bomb - crashes system"),
    ("dd if=/dev/zero of=/dev/sda", "Overwrites disk with zeros"),
    (
        "dd if=/dev/random of=/dev/sda",
        "Overwrites disk with random data",
    ),
    ("mkfs.", "Formats filesystem"),
    ("> /dev/sda", "Destroys disk data"),
    ("chmod -R 777 /", "Removes all file permissions"),
    ("chown -R", "Recursively changes ownership"),
];

const DANGEROUS_PATTERNS: &[(&str, &str)] = &[
    ("rm -rf", "Recursive forced deletion"),
    ("rm -fr", "Recursive forced deletion"),
    ("rm -r", "Recursive deletion"),
    ("rmdir", "Directory removal"),
    ("dd if=", "Low-level disk operation"),
    ("mkfs", "Filesystem format"),
    ("fdisk", "Disk partitioning"),
    ("parted", "Disk partitioning"),
    ("kill -9", "Force kill process"),
    ("killall", "Kill multiple processes"),
    ("pkill", "Kill processes by pattern"),
    ("shutdown", "System shutdown"),
    ("reboot", "System reboot"),
    ("init 0", "System halt"),
    ("init 6", "System reboot"),
    ("halt", "System halt"),
    ("poweroff", "System poweroff"),
    (":(){", "Potential fork bomb"),
    ("| sh", "Piping to shell"),
    ("| bash", "Piping to shell"),
    ("| zsh", "Piping to shell"),
    ("curl | sh", "Remote code execution"),
    ("wget | sh", "Remote code execution"),
    ("curl | bash", "Remote code execution"),
    ("wget | bash", "Remote code execution"),
    ("> /dev/", "Writing to device"),
    ("chmod 777", "Overly permissive permissions"),
    ("chmod -R", "Recursive permission change"),
    ("sudo rm", "Privileged deletion"),
    ("sudo dd", "Privileged disk operation"),
    ("sudo mkfs", "Privileged format"),
];

const WARNING_PATTERNS: &[(&str, &str)] = &[
    ("rm ", "File deletion"),
    ("mv ", "File move (can overwrite)"),
    ("cp -f", "Forced copy (overwrites)"),
    ("> ", "File overwrite via redirection"),
    (">>", "File append via redirection"),
    ("truncate", "File truncation"),
    ("shred", "Secure file deletion"),
    ("sudo", "Privileged command"),
    ("su ", "Switch user"),
    ("chmod", "Permission change"),
    ("chown", "Ownership change"),
    ("chgrp", "Group change"),
    ("iptables", "Firewall modification"),
    ("ufw", "Firewall modification"),
    ("systemctl stop", "Service stop"),
    ("systemctl disable", "Service disable"),
    ("service stop", "Service stop"),
    ("crontab -r", "Remove all cron jobs"),
    ("git reset --hard", "Discard git changes"),
    ("git clean -fd", "Remove untracked files"),
    ("git push --force", "Force push (rewrites history)"),
    ("docker rm", "Container removal"),
    ("docker rmi", "Image removal"),
    ("docker system prune", "Docker cleanup"),
    ("npm uninstall", "Package removal"),
    ("pip uninstall", "Package removal"),
    ("brew uninstall", "Package removal"),
    ("apt remove", "Package removal"),
    ("apt purge", "Package removal with config"),
    ("yum remove", "Package removal"),
    ("pacman -R", "Package removal"),
];

impl SafetyCheck {
    /// Analyze a command for potential dangers
    pub fn analyze(command: &str) -> Self {
        let cmd_lower = command.to_lowercase();
        let mut reasons = Vec::new();
        let mut max_severity = Severity::Safe;

        // Check for critical patterns first
        for (pattern, reason) in CRITICAL_COMMANDS {
            if cmd_lower.contains(&pattern.to_lowercase()) {
                reasons.push(format!("CRITICAL: {}", reason));
                max_severity = Severity::Critical;
            }
        }

        // Check for dangerous patterns
        for (pattern, reason) in DANGEROUS_PATTERNS {
            if cmd_lower.contains(&pattern.to_lowercase()) {
                reasons.push(format!("DANGER: {}", reason));
                if max_severity < Severity::Dangerous {
                    max_severity = Severity::Dangerous;
                }
            }
        }

        // Check for warning patterns
        for (pattern, reason) in WARNING_PATTERNS {
            if cmd_lower.contains(&pattern.to_lowercase()) {
                // Avoid duplicate reasons
                let warning = format!("WARNING: {}", reason);
                if !reasons.iter().any(|r| r.contains(reason)) {
                    reasons.push(warning);
                    if max_severity < Severity::Warning {
                        max_severity = Severity::Warning;
                    }
                }
            }
        }

        // Additional heuristic checks
        Self::check_heuristics(command, &mut reasons, &mut max_severity);

        // Deduplicate reasons
        let unique_reasons: Vec<String> = reasons
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        SafetyCheck {
            is_destructive: max_severity >= Severity::Warning,
            reasons: unique_reasons,
            severity: max_severity,
        }
    }

    fn check_heuristics(command: &str, reasons: &mut Vec<String>, severity: &mut Severity) {
        // Check for commands with wildcards in dangerous contexts
        if command.contains("rm ") && command.contains("*") {
            reasons.push("WARNING: Wildcard deletion".to_string());
            if *severity < Severity::Warning {
                *severity = Severity::Warning;
            }
        }

        // Check for recursive operations on root or home
        if (command.contains("-r") || command.contains("-R"))
            && (command.contains(" /") || command.contains(" ~/"))
        {
            reasons.push("DANGER: Recursive operation on system/home directory".to_string());
            if *severity < Severity::Dangerous {
                *severity = Severity::Dangerous;
            }
        }

        // Check for environment variable manipulation
        if command.contains("export PATH=") && !command.contains("$PATH") {
            reasons.push("WARNING: PATH override (may break shell)".to_string());
            if *severity < Severity::Warning {
                *severity = Severity::Warning;
            }
        }

        // Check for history manipulation
        if command.contains("history -c") || command.contains("HISTFILE=/dev/null") {
            reasons.push("WARNING: History manipulation (suspicious)".to_string());
            if *severity < Severity::Warning {
                *severity = Severity::Warning;
            }
        }
    }

    /// Returns true if this command should always require confirmation
    pub fn requires_confirmation(&self) -> bool {
        self.severity >= Severity::Warning
    }

    /// Returns true if this command should be blocked entirely
    pub fn should_block(&self) -> bool {
        self.severity == Severity::Critical
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Safe => write!(f, "safe"),
            Severity::Warning => write!(f, "warning"),
            Severity::Dangerous => write!(f, "dangerous"),
            Severity::Critical => write!(f, "critical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_rm_rf_root() {
        let check = SafetyCheck::analyze("rm -rf /");
        assert!(check.is_destructive);
        assert_eq!(check.severity, Severity::Critical);
    }

    #[test]
    fn detects_rm_rf() {
        let check = SafetyCheck::analyze("rm -rf ./node_modules");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Dangerous);
    }

    #[test]
    fn detects_simple_rm() {
        let check = SafetyCheck::analyze("rm file.txt");
        assert!(check.is_destructive);
        assert_eq!(check.severity, Severity::Warning);
    }

    #[test]
    fn detects_curl_pipe_sh() {
        let check = SafetyCheck::analyze("curl https://example.com/script.sh | sh");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Dangerous);
    }

    #[test]
    fn safe_command_passes() {
        let check = SafetyCheck::analyze("ls -la");
        assert!(!check.is_destructive);
        assert_eq!(check.severity, Severity::Safe);
    }

    #[test]
    fn detects_sudo() {
        let check = SafetyCheck::analyze("sudo apt update");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Warning);
    }

    #[test]
    fn detects_git_force_push() {
        let check = SafetyCheck::analyze("git push --force origin main");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Warning);
    }
}
