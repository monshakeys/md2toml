use std::collections::BTreeMap;
use crate::error::{Result, Md2TomlError};

/// Parsed markdown structure containing metadata and content
///
/// Uses `BTreeMap` for automatic alphabetical sorting of metadata keys.
pub struct ParsedMarkdown {
    pub metadata: BTreeMap<String, String>,
    pub content: String,
}

/// Parses frontmatter and content in a single pass
///
/// Avoids redundant scans by finding the `---` delimiter only once.
pub fn parse_markdown(markdown: &str) -> Result<ParsedMarkdown> {
    if !markdown.starts_with("---") {
        return Ok(ParsedMarkdown {
            metadata: BTreeMap::new(),
            content: markdown.to_string(),
        });
    }

    let lines: Vec<&str> = markdown.lines().collect();
    if lines.is_empty() {
        return Ok(ParsedMarkdown {
            metadata: BTreeMap::new(),
            content: String::new(),
        });
    }

    let mut end_index = None;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            end_index = Some(i);
            break;
        }
    }

    let end_index = match end_index {
        Some(idx) => idx,
        None => {
            return Ok(ParsedMarkdown {
                metadata: BTreeMap::new(),
                content: markdown.to_string(),
            });
        }
    };

    let frontmatter_lines = &lines[1..end_index];
    let yaml_content = frontmatter_lines.join("\n");

    let metadata = if yaml_content.trim().is_empty() {
        BTreeMap::new()
    } else {
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)?;
        let mut result = BTreeMap::new();

        if let serde_yaml::Value::Mapping(map) = yaml_value {
            for (key, value) in map {
                if let serde_yaml::Value::String(k) = key {
                    let v = match value {
                        serde_yaml::Value::String(s) => s,
                        serde_yaml::Value::Number(n) => n.to_string(),
                        serde_yaml::Value::Bool(b) => b.to_string(),
                        _ => value.as_str().unwrap_or("").to_string(),
                    };
                    result.insert(k, v);
                }
            }
        }
        result
    };

    let mut char_pos = 0;
    for (i, line) in lines.iter().enumerate() {
        if i <= end_index {
            char_pos += line.len() + 1;
        } else {
            break;
        }
    }

    let content = if char_pos < markdown.len() {
        &markdown[char_pos..]
    } else {
        ""
    };

    let content = if content.starts_with('\n') {
        &content[1..]
    } else {
        content
    };

    let content = content.strip_suffix('\n').unwrap_or(content);

    Ok(ParsedMarkdown {
        metadata,
        content: content.to_string(),
    })
}

/// Extracts YAML frontmatter metadata from markdown
///
/// Maintained for backward compatibility; internally calls `parse_markdown`.
pub fn parse_frontmatter(content: &str) -> Result<BTreeMap<String, String>> {
    let parsed = parse_markdown(content)?;
    Ok(parsed.metadata)
}

/// Extracts content after frontmatter
///
/// Maintained for backward compatibility; internally calls `parse_markdown`.
pub fn extract_content(markdown: &str) -> Result<String> {
    let parsed = parse_markdown(markdown)?;
    Ok(parsed.content)
}

/// Validate that metadata contains required fields for cc slash command
/// Required fields: "title" and "description" (case-sensitive)
pub fn validate_required_fields(metadata: &BTreeMap<String, String>) -> Result<()> {
    if !metadata.contains_key("title") || !metadata.contains_key("description") {
        return Err(Md2TomlError::InvalidFrontmatter(
            "This is not a markdown file for cc slash command.".to_string()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_frontmatter_with_simple_metadata() {
        let input = r#"---
title: Simple Example
author: Test User
version: 1.0
---

Content here"#;

        let result = parse_frontmatter(input).unwrap();

        assert_eq!(result.get("title").unwrap(), "Simple Example");
        assert_eq!(result.get("author").unwrap(), "Test User");
        assert_eq!(result.get("version").unwrap(), "1.0");
    }

    #[test]
    fn test_parse_frontmatter_with_empty_metadata() {
        let input = r#"---
---

Content here"#;

        let result = parse_frontmatter(input).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_frontmatter_with_no_metadata() {
        let input = "Just content, no frontmatter";

        let result = parse_frontmatter(input).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_frontmatter_with_quotes_in_values() {
        let input = r#"---
title: "Quoted Title"
description: 'Single quotes'
mixed: Mix "of" quotes
---

Content"#;

        let result = parse_frontmatter(input).unwrap();

        assert_eq!(result.get("title").unwrap(), "Quoted Title");
        assert_eq!(result.get("description").unwrap(), "Single quotes");
        assert_eq!(result.get("mixed").unwrap(), "Mix \"of\" quotes");
    }

    #[test]
    fn test_parse_frontmatter_with_special_characters() {
        let input = r#"---
path: /path/to/file
url: https://example.com/page?param=value
tags: test, complex, full-featured
---

Content"#;

        let result = parse_frontmatter(input).unwrap();

        assert_eq!(result.get("path").unwrap(), "/path/to/file");
        assert_eq!(result.get("url").unwrap(), "https://example.com/page?param=value");
        assert_eq!(result.get("tags").unwrap(), "test, complex, full-featured");
    }

    #[test]
    fn test_parse_frontmatter_with_boolean_and_numbers() {
        let input = r#"---
enabled: true
count: 42
price: 19.99
active: false
---

Content"#;

        let result = parse_frontmatter(input).unwrap();

        assert_eq!(result.get("enabled").unwrap(), "true");
        assert_eq!(result.get("count").unwrap(), "42");
        assert_eq!(result.get("price").unwrap(), "19.99");
        assert_eq!(result.get("active").unwrap(), "false");
    }

    #[test]
    fn test_extract_content_simple() {
        let input = r#"---
title: Test
---

This is content.
With multiple lines."#;

        let result = extract_content(input).unwrap();

        assert_eq!(result, "This is content.\nWith multiple lines.");
    }

    #[test]
    fn test_extract_content_preserves_newlines() {
        let input = r#"---
title: Test
---

Line 1

Line 3 with blank line above"#;

        let result = extract_content(input).unwrap();

        assert_eq!(result, "Line 1\n\nLine 3 with blank line above");
    }

    #[test]
    fn test_extract_content_preserves_indentation() {
        let input = r#"---
title: Test
---

Normal line
  Indented line
    More indented"#;

        let result = extract_content(input).unwrap();

        assert_eq!(result, "Normal line\n  Indented line\n    More indented");
    }

    #[test]
    fn test_extract_content_when_empty() {
        let input = r#"---
title: Test
---
"#;

        let result = extract_content(input).unwrap();

        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_content_with_no_frontmatter() {
        let input = "Just content here\nNo frontmatter at all";

        let result = extract_content(input).unwrap();

        assert_eq!(result, "Just content here\nNo frontmatter at all");
    }

    #[test]
    fn test_extract_content_with_special_characters() {
        let input = r#"---
title: Test
---

Content with $ARGUMENTS and !`commands`
Special chars: @#$%^&*()"#;

        let result = extract_content(input).unwrap();

        assert_eq!(result, "Content with $ARGUMENTS and !`commands`\nSpecial chars: @#$%^&*()");
    }

    #[test]
    fn test_extract_content_preserves_trailing_newlines() {
        let input = "---\ntitle: Test\n---\n\nContent\n\n\n";

        let result = extract_content(input).unwrap();

        assert_eq!(result, "Content\n\n");
    }

    #[test]
    fn test_validate_required_fields_with_both_fields() {
        let mut metadata = BTreeMap::new();
        metadata.insert("title".to_string(), "Test Title".to_string());
        metadata.insert("description".to_string(), "Test Description".to_string());
        metadata.insert("author".to_string(), "Optional".to_string());

        let result = validate_required_fields(&metadata);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_required_fields_missing_title() {
        let mut metadata = BTreeMap::new();
        metadata.insert("description".to_string(), "Test Description".to_string());

        let result = validate_required_fields(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid frontmatter format: This is not a markdown file for cc slash command."
        );
    }

    #[test]
    fn test_validate_required_fields_missing_description() {
        let mut metadata = BTreeMap::new();
        metadata.insert("title".to_string(), "Test Title".to_string());

        let result = validate_required_fields(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid frontmatter format: This is not a markdown file for cc slash command."
        );
    }

    #[test]
    fn test_validate_required_fields_missing_both() {
        let metadata = BTreeMap::new();

        let result = validate_required_fields(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid frontmatter format: This is not a markdown file for cc slash command."
        );
    }

    #[test]
    fn test_validate_required_fields_case_sensitive() {
        let mut metadata = BTreeMap::new();
        metadata.insert("Title".to_string(), "Test".to_string());
        metadata.insert("Description".to_string(), "Test".to_string());

        let result = validate_required_fields(&metadata);
        assert!(result.is_err());
    }
}
