use std::collections::BTreeMap;
use crate::error::Result;

/// Generates TOML output from metadata and prompt content.
///
/// Uses BTreeMap for automatic key ordering and pre-allocates string capacity
/// for optimal performance.
pub fn generate_toml(metadata: BTreeMap<String, String>, prompt: String) -> Result<String> {
    let estimated_size = metadata.len() * 50 + prompt.len() + 100;
    let mut output = String::with_capacity(estimated_size);

    for (key, value) in &metadata {
        let escaped_value = value.replace('"', "\\\"");
        output.push_str(&format!("{} = \"{}\"\n", key, escaped_value));
    }

    if !metadata.is_empty() {
        output.push('\n');
    }

    output.push_str("prompt = \"\"\"\n");
    output.push_str(&prompt);

    if !prompt.is_empty() && !prompt.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("\"\"\"\n");

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_toml_simple() {
        let mut metadata = BTreeMap::new();
        metadata.insert("title".to_string(), "Test".to_string());
        metadata.insert("author".to_string(), "User".to_string());

        let prompt = "Simple content".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        assert!(result.contains("title = \"Test\""));
        assert!(result.contains("author = \"User\""));
        assert!(result.contains("prompt = \"\"\""));
        assert!(result.contains("Simple content"));
    }

    #[test]
    fn test_generate_toml_with_empty_prompt() {
        let mut metadata = BTreeMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        let prompt = "".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        assert!(result.contains("key = \"value\""));
        assert!(result.contains("prompt = \"\"\""));
    }

    #[test]
    fn test_generate_toml_multiline_prompt() {
        let mut metadata = BTreeMap::new();
        metadata.insert("name".to_string(), "Test".to_string());

        let prompt = "Line 1\nLine 2\n\nLine 4".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        assert!(result.contains("name = \"Test\""));
        assert!(result.contains("prompt = \"\"\""));
        assert!(result.contains("Line 1\nLine 2\n\nLine 4"));
    }

    #[test]
    fn test_generate_toml_special_chars_in_metadata() {
        let mut metadata = BTreeMap::new();
        metadata.insert("path".to_string(), "/path/to/file".to_string());
        metadata.insert("url".to_string(), "https://example.com".to_string());
        metadata.insert("quote".to_string(), "He said \"hello\"".to_string());

        let prompt = "Content".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        assert!(result.contains("path = \"/path/to/file\""));
        assert!(result.contains("url = \"https://example.com\""));
        // Escaped quotes in TOML
        assert!(result.contains("quote = \"He said \\\"hello\\\"\"") || result.contains("quote = 'He said \"hello\"'"));
    }

    #[test]
    fn test_generate_toml_special_chars_in_prompt() {
        let metadata = BTreeMap::new();

        let prompt = "Content with \"quotes\" and \\ backslashes".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        assert!(result.contains("prompt = \"\"\""));
        // Multi-line strings in TOML preserve content literally
        assert!(result.contains("Content with \"quotes\" and \\ backslashes"));
    }

    #[test]
    fn test_generate_toml_preserves_indentation() {
        let metadata = BTreeMap::new();

        let prompt = "Normal\n  Indented\n    More indented".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        assert!(result.contains("Normal\n  Indented\n    More indented"));
    }

    #[test]
    fn test_generate_toml_with_no_metadata() {
        let metadata = BTreeMap::new();
        let prompt = "Only prompt content".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        assert!(result.contains("prompt = \"\"\""));
        assert!(result.contains("Only prompt content"));
    }

    #[test]
    fn test_generate_toml_metadata_ordering() {
        let mut metadata = BTreeMap::new();
        metadata.insert("zebra".to_string(), "last".to_string());
        metadata.insert("alpha".to_string(), "first".to_string());
        metadata.insert("beta".to_string(), "second".to_string());

        let prompt = "Test".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        // All metadata should appear before prompt
        let prompt_pos = result.find("prompt = ").unwrap();
        let alpha_pos = result.find("alpha = ").unwrap();
        let beta_pos = result.find("beta = ").unwrap();
        let zebra_pos = result.find("zebra = ").unwrap();

        assert!(alpha_pos < prompt_pos);
        assert!(beta_pos < prompt_pos);
        assert!(zebra_pos < prompt_pos);
    }

    #[test]
    fn test_generate_toml_complex() {
        let mut metadata = BTreeMap::new();
        metadata.insert("title".to_string(), "Complex".to_string());
        metadata.insert("version".to_string(), "1.0".to_string());

        let prompt = "Line 1\n\nLine 3 with {{args}}\nAnd !{command}".to_string();

        let result = generate_toml(metadata, prompt).unwrap();

        assert!(result.contains("title = \"Complex\""));
        assert!(result.contains("version = \"1.0\""));
        assert!(result.contains("Line 1\n\nLine 3 with {{args}}\nAnd !{command}"));
    }
}
