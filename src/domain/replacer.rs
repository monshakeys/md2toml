//! Domain Layer - Text Replacement Logic
//!
//! Pure business logic for text transformations.

use crate::domain::ports::RegexPort;

/// Simple string-based argument replacement
/// Replaces $ARGUMENTS placeholder with {{args}} template syntax
pub fn replace_arguments(content: &str) -> String {
    content.replace("$ARGUMENTS", "{{args}}")
}

/// Text replacement service that uses dependency injection
pub struct TextReplacer {
    regex_service: Box<dyn RegexPort>,
}

impl TextReplacer {
    /// Create new text replacer with regex service dependency
    pub fn new(regex_service: Box<dyn RegexPort>) -> Self {
        Self { regex_service }
    }

    /// Replaces !`command` patterns with !{command} syntax
    pub fn replace_backtick_patterns(&self, content: &str) -> String {
        self.regex_service.replace_backtick_patterns(content)
    }

    /// Applies all text replacements for slash command conversion
    pub fn apply_all_replacements(&self, content: &str) -> String {
        let step1 = replace_arguments(content);
        self.replace_backtick_patterns(&step1)
    }
}

/// Applies all text replacements using RegexService internally
///
/// This is a convenience function that creates a RegexService instance
/// and applies both $ARGUMENTS and !`...` replacements.
///
/// Uses the same regex-based logic as TextReplacer for correctness.
pub fn apply_all_replacements_simple(content: &str) -> String {
    let step1 = replace_arguments(content);

    // RegexService is zero-cost (ZST) and regex is compiled statically
    let regex_service = crate::infra::RegexService::new();
    use crate::domain::ports::RegexPort;
    regex_service.replace_backtick_patterns(&step1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRegexPort;
    impl RegexPort for MockRegexPort {
        fn replace_backtick_patterns(&self, content: &str) -> String {
            // Use the real RegexService for testing (it's zero-cost)
            let service = crate::infra::RegexService::new();
            service.replace_backtick_patterns(content)
        }
    }

    #[test]
    fn test_replace_arguments_single_occurrence() {
        let input = "Please process these $ARGUMENTS carefully.";
        let result = replace_arguments(input);
        assert_eq!(result, "Please process these {{args}} carefully.");
    }

    #[test]
    fn test_replace_arguments_multiple_occurrences() {
        let input = "First $ARGUMENTS and second $ARGUMENTS in line.";
        let result = replace_arguments(input);
        assert_eq!(result, "First {{args}} and second {{args}} in line.");
    }

    #[test]
    fn test_replace_arguments_multiline() {
        let input = "Line 1 with $ARGUMENTS\nLine 2 with $ARGUMENTS\nLine 3";
        let result = replace_arguments(input);
        assert_eq!(result, "Line 1 with {{args}}\nLine 2 with {{args}}\nLine 3");
    }

    #[test]
    fn test_text_replacer_with_mock_regex() {
        let replacer = TextReplacer::new(Box::new(MockRegexPort));
        let input = "Process $ARGUMENTS with !`command` here.";
        let result = replacer.apply_all_replacements(input);
        assert_eq!(result, "Process {{args}} with !{command} here.");
    }

    #[test]
    fn test_apply_all_replacements_simple() {
        let input = "First $ARGUMENTS and second $ARGUMENTS in line.";
        let result = apply_all_replacements_simple(input);
        assert_eq!(result, "First {{args}} and second {{args}} in line.");
    }

    #[test]
    fn test_apply_all_replacements_simple_with_normal_backticks() {
        // Critical test: Ensures normal backticks are NOT replaced
        let input = "Use `git status` to check and !`git commit` to save.";
        let result = apply_all_replacements_simple(input);
        assert_eq!(
            result, "Use `git status` to check and !{git commit} to save.",
            "Normal backticks should be preserved, only !`...` should be replaced"
        );
    }

    #[test]
    fn test_apply_all_replacements_simple_mixed_content() {
        // Real-world scenario: technical documentation with inline code
        let input = "Check `npm install` then run !`{{args}}` for the task.";
        let result = apply_all_replacements_simple(input);
        assert_eq!(
            result,
            "Check `npm install` then run !{{{args}}} for the task."
        );
    }

    #[test]
    fn test_apply_all_replacements_simple_multiple_patterns() {
        // Multiple normal backticks and special patterns
        let input = "Functions: `foo()`, `bar()`, commands: !`cmd1`, !`cmd2`.";
        let result = apply_all_replacements_simple(input);
        assert_eq!(
            result,
            "Functions: `foo()`, `bar()`, commands: !{cmd1}, !{cmd2}."
        );
    }

    #[test]
    fn test_apply_all_replacements_simple_with_arguments() {
        // Combined: $ARGUMENTS + mixed backticks
        let input = "Process $ARGUMENTS using `tool` or !`command`.";
        let result = apply_all_replacements_simple(input);
        assert_eq!(result, "Process {{args}} using `tool` or !{command}.");
    }
}
