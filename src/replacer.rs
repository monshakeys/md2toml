use regex::Regex;
use once_cell::sync::Lazy;

/// Lazily compiled regex for backtick pattern replacement.
/// Initialized once at startup for performance.
static BACKTICK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"!`([^`]*)`").expect("Failed to compile backtick regex")
});

/// Replaces $ARGUMENTS placeholder with {{args}} template syntax
pub fn replace_arguments(content: &str) -> String {
    content.replace("$ARGUMENTS", "{{args}}")
}

/// Replaces !`command` patterns with !{command} syntax
pub fn replace_backtick_patterns(content: &str) -> String {
    BACKTICK_REGEX.replace_all(content, "!{$1}").to_string()
}

/// Applies all text replacements for slash command conversion
pub fn apply_all_replacements(content: &str) -> String {
    let step1 = replace_arguments(content);
    let step2 = replace_backtick_patterns(&step1);
    step2
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // Tests for $ARGUMENTS replacement
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
    fn test_replace_arguments_no_occurrences() {
        let input = "No replacements needed here.";
        let result = replace_arguments(input);
        assert_eq!(result, "No replacements needed here.");
    }

    #[test]
    fn test_replace_arguments_multiline() {
        let input = "Line 1 with $ARGUMENTS\nLine 2 with $ARGUMENTS\nLine 3";
        let result = replace_arguments(input);
        assert_eq!(result, "Line 1 with {{args}}\nLine 2 with {{args}}\nLine 3");
    }

    #[test]
    fn test_replace_arguments_case_sensitive() {
        let input = "Test $arguments and $Arguments and $ARGUMENTS";
        let result = replace_arguments(input);
        // Only $ARGUMENTS should be replaced (case-sensitive)
        assert_eq!(result, "Test $arguments and $Arguments and {{args}}");
    }

    // Tests for backtick pattern replacement
    #[test]
    fn test_replace_backtick_patterns_simple() {
        let input = "Run command !`ls -la` to list files.";
        let result = replace_backtick_patterns(input);
        assert_eq!(result, "Run command !{ls -la} to list files.");
    }

    #[test]
    fn test_replace_backtick_patterns_multiple() {
        let input = "Use !`echo hello` or !`cat file.txt` commands.";
        let result = replace_backtick_patterns(input);
        assert_eq!(result, "Use !{echo hello} or !{cat file.txt} commands.");
    }

    #[test]
    fn test_replace_backtick_patterns_complex() {
        let input = "Pattern !`grep -r \"test\" .` is complex.";
        let result = replace_backtick_patterns(input);
        assert_eq!(result, "Pattern !{grep -r \"test\" .} is complex.");
    }

    #[test]
    fn test_replace_backtick_patterns_with_dots() {
        let input = "Command !`complex.command.here` works.";
        let result = replace_backtick_patterns(input);
        assert_eq!(result, "Command !{complex.command.here} works.");
    }

    #[test]
    fn test_replace_backtick_patterns_no_occurrences() {
        let input = "No backtick patterns here.";
        let result = replace_backtick_patterns(input);
        assert_eq!(result, "No backtick patterns here.");
    }

    #[test]
    fn test_replace_backtick_patterns_multiline() {
        let input = "Line 1: !`cmd1`\nLine 2: !`cmd2`";
        let result = replace_backtick_patterns(input);
        assert_eq!(result, "Line 1: !{cmd1}\nLine 2: !{cmd2}");
    }

    #[test]
    fn test_replace_backtick_patterns_empty_content() {
        let input = "Empty pattern !`` should work.";
        let result = replace_backtick_patterns(input);
        assert_eq!(result, "Empty pattern !{} should work.");
    }

    #[test]
    fn test_replace_backtick_patterns_with_spaces() {
        let input = "Pattern !`git commit -m \"message\"` with spaces.";
        let result = replace_backtick_patterns(input);
        assert_eq!(result, "Pattern !{git commit -m \"message\"} with spaces.");
    }

    // Tests for apply_all_replacements
    #[test]
    fn test_apply_all_replacements_both_patterns() {
        let input = "Process $ARGUMENTS with !`command` here.";
        let result = apply_all_replacements(input);
        assert_eq!(result, "Process {{args}} with !{command} here.");
    }

    #[test]
    fn test_apply_all_replacements_multiple_of_both() {
        let input = "First $ARGUMENTS and !`cmd1`, then $ARGUMENTS and !`cmd2`.";
        let result = apply_all_replacements(input);
        assert_eq!(result, "First {{args}} and !{cmd1}, then {{args}} and !{cmd2}.");
    }

    #[test]
    fn test_apply_all_replacements_only_arguments() {
        let input = "Only $ARGUMENTS replacement needed.";
        let result = apply_all_replacements(input);
        assert_eq!(result, "Only {{args}} replacement needed.");
    }

    #[test]
    fn test_apply_all_replacements_only_backticks() {
        let input = "Only !`command` replacement needed.";
        let result = apply_all_replacements(input);
        assert_eq!(result, "Only !{command} replacement needed.");
    }

    #[test]
    fn test_apply_all_replacements_no_patterns() {
        let input = "No replacements needed at all.";
        let result = apply_all_replacements(input);
        assert_eq!(result, "No replacements needed at all.");
    }

    #[test]
    fn test_apply_all_replacements_complex_multiline() {
        let input = r#"Line 1: $ARGUMENTS here
Line 2: !`git status` command
Line 3: Both $ARGUMENTS and !`npm install`
Line 4: Nothing"#;
        let expected = r#"Line 1: {{args}} here
Line 2: !{git status} command
Line 3: Both {{args}} and !{npm install}
Line 4: Nothing"#;
        let result = apply_all_replacements(input);
        assert_eq!(result, expected);
    }
}
