use std::fs;
use std::path::PathBuf;

fn get_fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    let path = get_fixture_path(name);
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read fixture: {}", name))
}

fn convert_md_to_toml(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let metadata = md2toml::parse_frontmatter(input)?;
    let content = md2toml::extract_content(input)?;
    let processed = md2toml::apply_all_replacements_simple(&content);
    let toml = md2toml::generate_toml(&metadata, &processed)?;
    Ok(toml)
}

/// Converts markdown with validation (same as main.rs workflow)
fn convert_md_to_toml_with_validation(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let metadata = md2toml::parse_frontmatter(input)?;
    md2toml::validate_required_fields(&metadata)?;
    let content = md2toml::extract_content(input)?;
    let processed = md2toml::apply_all_replacements_simple(&content);
    let toml = md2toml::generate_toml(&metadata, &processed)?;
    Ok(toml)
}

/// Pure Logic Tests (Domain Logic, CPU-bound)
/// These tests operate entirely in memory with no external I/O.
/// According to Rust Guidelines 4-3: No timeout needed for pure logic tests.

#[test]
fn test_integration_simple() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("simple.md");
    let expected = read_fixture("simple.expected.toml");

    let result = convert_md_to_toml(&input).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_integration_with_arguments() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("with_arguments.md");
    let expected = read_fixture("with_arguments.expected.toml");

    let result = convert_md_to_toml(&input).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_integration_with_backticks() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("with_backticks.md");
    let expected = read_fixture("with_backticks.expected.toml");

    let result = convert_md_to_toml(&input).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_integration_complex() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("complex.md");
    let expected = read_fixture("complex.expected.toml");

    let result = convert_md_to_toml(&input).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_integration_empty_content() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("empty_content.md");
    let expected = read_fixture("empty_content.expected.toml");

    let result = convert_md_to_toml(&input).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_integration_no_metadata() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("no_metadata.md");
    let expected = read_fixture("no_metadata.expected.toml");

    let result = convert_md_to_toml(&input).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_validation_pass_with_required_fields() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("complex.md");
    let expected = read_fixture("complex.expected.toml");

    let result = convert_md_to_toml_with_validation(&input).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_validation_fail_missing_description() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("simple.md");

    let result = convert_md_to_toml_with_validation(&input);

    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("This is not a markdown file for cc slash command."));
}

#[test]
fn test_validation_fail_missing_title() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("with_arguments.md");

    let result = convert_md_to_toml_with_validation(&input);

    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("This is not a markdown file for cc slash command."));
}

#[test]
fn test_validation_fail_no_metadata() {
    // Pure Logic Test - No timeout required
    let input = read_fixture("no_metadata.md");

    let result = convert_md_to_toml_with_validation(&input);

    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("This is not a markdown file for cc slash command."));
}
