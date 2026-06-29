use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

/// Recognized highlight names — must match captures in .scm files.
const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "boolean",
    "comment",
    "comment.documentation",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "escape",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "keyword",
    "label",
    "number",
    "operator",
    "property",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

struct LanguageConfig {
    language: tree_sitter::Language,
    query: &'static str,
}

fn lang_config(lang: &str) -> Option<LanguageConfig> {
    match lang {
        "rust" | "rs" => Some(LanguageConfig {
            language: tree_sitter_rust::LANGUAGE.into(),
            query: include_str!("queries/rust.scm"),
        }),
        "python" | "py" => Some(LanguageConfig {
            language: tree_sitter_python::LANGUAGE.into(),
            query: include_str!("queries/python.scm"),
        }),
        "javascript" | "js" | "jsx" => Some(LanguageConfig {
            language: tree_sitter_javascript::LANGUAGE.into(),
            query: include_str!("queries/javascript.scm"),
        }),
        "html" => Some(LanguageConfig {
            language: tree_sitter_html::LANGUAGE.into(),
            query: include_str!("queries/html.scm"),
        }),
        "css" => Some(LanguageConfig {
            language: tree_sitter_css::LANGUAGE.into(),
            query: include_str!("queries/css.scm"),
        }),
        _ => None,
    }
}

/// Highlight source code using tree-sitter for the given language.
///
/// Returns HTML with `<span class="ts-{name}">` wrapped tokens.
/// Falls back to plain text if the language is unsupported or parsing fails.
pub fn highlight(source: &str, lang: &str) -> String {
    let lc = match lang_config(lang) {
        Some(c) => c,
        None => return html_escape(source),
    };

    let mut config = match HighlightConfiguration::new(
        lc.language,
        lang,
        lc.query,
        "",
        "",
    ) {
        Ok(c) => c,
        Err(_) => return html_escape(source),
    };
    config.configure(HIGHLIGHT_NAMES);

    let mut highlighter = Highlighter::new();
    let events = match highlighter.highlight(&config, source.as_bytes(), None, |_| None) {
        Ok(e) => e,
        Err(_) => return html_escape(source),
    };

    let mut renderer = HtmlRenderer::new();
    let _ = renderer.render(events, source.as_bytes(), &|highlight, out| {
        let idx = highlight.0;
        if let Some(name) = HIGHLIGHT_NAMES.get(idx) {
            out.extend_from_slice(b"class=\"ts-");
            out.extend_from_slice(name.as_bytes());
            out.extend_from_slice(b"\"");
        }
    });

    let html = String::from_utf8_lossy(&renderer.html);
    html.trim_end().to_string()
}

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
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-function"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_python() {
        let source = r#"def hello():
    print("world")"#;
        let result = highlight(source, "python");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-function"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_javascript() {
        let source = r#"function greet(name) {
    return "hi";
}"#;
        let result = highlight(source, "javascript");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-function"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_html() {
        let source = r#"<div class="foo">hello</div>"#;
        let result = highlight(source, "html");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-tag"));
        assert!(result.contains("ts-attribute"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_css() {
        let source = r#"body { color: red; }"#;
        let result = highlight(source, "css");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-tag"));
        assert!(result.contains("ts-property"));
    }

    #[test]
    fn test_highlight_unknown_lang() {
        let result = highlight("x = 1", "unknown");
        assert!(!result.contains("<span"));
        assert_eq!(result, "x = 1");
    }
}
