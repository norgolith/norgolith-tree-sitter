use tree_sitter::Parser;

/// Supported languages for syntax highlighting
enum Language {
    Rust,
    Python,
    JavaScript,
    Html,
    Css,
}

/// Map language string to Language enum
fn resolve_language(lang: &str) -> Option<Language> {
    match lang {
        "rust" | "rs" => Some(Language::Rust),
        "python" | "py" => Some(Language::Python),
        "javascript" | "js" | "jsx" => Some(Language::JavaScript),
        "html" => Some(Language::Html),
        "css" => Some(Language::Css),
        _ => None,
    }
}

/// Highlight source code using tree-sitter for the given language.
///
/// Returns HTML with `<span class="ts-{kind}">` wrapped tokens.
/// Falls back to plain text if the language is unsupported or parsing fails.
pub fn highlight(source: &str, lang: &str) -> String {
    let language = match resolve_language(lang) {
        Some(l) => l,
        None => return html_escape(source),
    };

    let mut parser = Parser::new();
    match &language {
        Language::Rust => {
            if parser.set_language(&tree_sitter_rust::LANGUAGE.into()).is_err() {
                return html_escape(source);
            }
        }
        Language::Python => {
            if parser.set_language(&tree_sitter_python::LANGUAGE.into()).is_err() {
                return html_escape(source);
            }
        }
        Language::JavaScript => {
            if parser.set_language(&tree_sitter_javascript::LANGUAGE.into()).is_err() {
                return html_escape(source);
            }
        }
        Language::Html => {
            if parser.set_language(&tree_sitter_html::LANGUAGE.into()).is_err() {
                return html_escape(source);
            }
        }
        Language::Css => {
            if parser.set_language(&tree_sitter_css::LANGUAGE.into()).is_err() {
                return html_escape(source);
            }
        }
    }

    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return html_escape(source),
    };

    let root = tree.root_node();
    let mut output = String::with_capacity(source.len() * 2);
    let mut cursor = root.walk();
    let mut pos = 0;

    walk_tree(source, &mut output, &mut cursor, &mut pos);

    output
}

/// Walk the CST using a cursor, producing highlighted HTML.
///
/// Algorithm from Gabriel Sanches' blog post on tree-sitter-on-the-web,
/// transcribed from Go to Rust.
fn walk_tree(source: &str, output: &mut String, cursor: &mut tree_sitter::TreeCursor, pos: &mut usize) {
    loop {
        let node = cursor.node();

        // If this node has named children, descend into them
        if node.named_child_count() > 0 && cursor.goto_first_child() {
            continue;
        }

        // This is a leaf node
        let range = node.byte_range();

        // Write any text that appears before this node (whitespace, punctuation)
        if range.start > *pos {
            let between = &source[*pos..range.start];
            output.push_str(&html_escape(between));
        }

        // Write the node itself wrapped in a span
        let value = &source[range.start..range.end];
        output.push_str(&format!(
            r#"<span class="ts-{}">{}</span>"#,
            node.kind(),
            html_escape(value)
        ));

        *pos = range.end;

        // Try to move to the next sibling
        if cursor.goto_next_sibling() {
            continue;
        }

        // Backtrack to find a parent with a sibling
        loop {
            if !cursor.goto_parent() {
                // Reached the root, we're done
                return;
            }
            if cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// HTML-escape a string for safe output
fn html_escape(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            '"' => result.push_str("&quot;"),
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("fn main() {}"), "fn main() {}");
        assert_eq!(html_escape("<div>"), "&lt;div&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
    }

    #[test]
    fn test_highlight_rust() {
        let source = r#"fn main() {
    println!("hello");
}"#;
        let result = highlight(source, "rust");
        assert!(result.contains("<span class="));
        assert!(result.contains("fn"));
        assert!(result.contains("main"));
    }

    #[test]
    fn test_highlight_unknown_lang() {
        let result = highlight("x = 1", "unknown");
        assert!(!result.contains("<span"));
        assert_eq!(result, "x = 1");
    }
}
