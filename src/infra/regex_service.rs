//! Infrastructure Layer - Regex Service Implementation
//!
//! Concrete implementation of RegexPort using regex crate.

use crate::domain::ports::RegexPort;
use once_cell::sync::Lazy;
use regex::Regex;

/// Regex service implementation
///
/// Uses lazy-loaded compiled regex for performance.
/// Initialized once at startup.
pub struct RegexService;

impl RegexService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RegexService {
    fn default() -> Self {
        Self::new()
    }
}

/// Lazily compiled regex for backtick pattern replacement.
/// Initialized once at startup for performance.
static BACKTICK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"!`([^`]*)`").expect("Failed to compile backtick regex"));

impl RegexPort for RegexService {
    fn replace_backtick_patterns(&self, content: &str) -> String {
        BACKTICK_REGEX.replace_all(content, "!{$1}").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_backtick_patterns_simple() {
        let service = RegexService::new();
        let input = "Run command !`ls -la` to list files.";
        let result = service.replace_backtick_patterns(input);
        assert_eq!(result, "Run command !{ls -la} to list files.");
    }

    #[test]
    fn test_replace_backtick_patterns_multiple() {
        let service = RegexService::new();
        let input = "Use !`echo hello` or !`cat file.txt` commands.";
        let result = service.replace_backtick_patterns(input);
        assert_eq!(result, "Use !{echo hello} or !{cat file.txt} commands.");
    }

    #[test]
    fn test_replace_backtick_patterns_no_occurrences() {
        let service = RegexService::new();
        let input = "No backtick patterns here.";
        let result = service.replace_backtick_patterns(input);
        assert_eq!(result, "No backtick patterns here.");
    }

    #[test]
    fn test_replace_backtick_patterns_multiline() {
        let service = RegexService::new();
        let input = "Line 1: !`cmd1`\nLine 2: !`cmd2`";
        let result = service.replace_backtick_patterns(input);
        assert_eq!(result, "Line 1: !{cmd1}\nLine 2: !{cmd2}");
    }
}