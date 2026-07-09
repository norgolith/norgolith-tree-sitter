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
    "string.special.key",
    "string.special.regex",
    "string.special.symbol",
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
        "bash" | "sh" | "shell" => Some(LanguageConfig {
            language: tree_sitter_bash::LANGUAGE.into(),
            query: tree_sitter_bash::HIGHLIGHT_QUERY,
        }),
        "nix" => Some(LanguageConfig {
            language: tree_sitter_nix::LANGUAGE.into(),
            query: tree_sitter_nix::HIGHLIGHTS_QUERY,
        }),
        "elixir" | "ex" | "exs" => Some(LanguageConfig {
            language: tree_sitter_elixir::LANGUAGE.into(),
            query: tree_sitter_elixir::HIGHLIGHTS_QUERY,
        }),
        "typescript" | "ts" => Some(LanguageConfig {
            language: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            query: tree_sitter_typescript::HIGHLIGHTS_QUERY,
        }),
        "markdown" | "md" => Some(LanguageConfig {
            language: tree_sitter_md::LANGUAGE.into(),
            query: tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
        }),
        "c" => Some(LanguageConfig {
            language: tree_sitter_c::LANGUAGE.into(),
            query: tree_sitter_c::HIGHLIGHT_QUERY,
        }),
        "c++" | "cpp" | "cxx" => Some(LanguageConfig {
            language: tree_sitter_cpp::LANGUAGE.into(),
            query: tree_sitter_cpp::HIGHLIGHT_QUERY,
        }),
        "java" => Some(LanguageConfig {
            language: tree_sitter_java::LANGUAGE.into(),
            query: tree_sitter_java::HIGHLIGHTS_QUERY,
        }),
        "json" => Some(LanguageConfig {
            language: tree_sitter_json::LANGUAGE.into(),
            query: tree_sitter_json::HIGHLIGHTS_QUERY,
        }),
        "yaml" | "yml" => Some(LanguageConfig {
            language: tree_sitter_yaml::LANGUAGE.into(),
            query: tree_sitter_yaml::HIGHLIGHTS_QUERY,
        }),
        "toml" => Some(LanguageConfig {
            language: tree_sitter_toml_updated::language().into(),
            query: tree_sitter_toml_updated::HIGHLIGHT_QUERY,
        }),
        "ruby" | "rb" => Some(LanguageConfig {
            language: tree_sitter_ruby::LANGUAGE.into(),
            query: tree_sitter_ruby::HIGHLIGHTS_QUERY,
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
    fn test_highlight_bash() {
        let source = r#"#!/usr/bin/env bash
for f in *.txt; do
    echo "$f"
done"#;
        let result = highlight(source, "bash");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-function"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_nix() {
        let source = r#"{ pkgs }:
with pkgs;
let x = 1;
in stdenv.mkDerivation {
    name = "hello";
}"#;
        let result = highlight(source, "nix");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_elixir() {
        let source = r#"defmodule Hello do
    def greet(name) do
        "Hello, #{name}"
    end
end"#;
        let result = highlight(source, "elixir");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_typescript() {
        let source = r#"interface Foo {
    name: string;
}"#;
        let result = highlight(source, "typescript");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-type"));
    }

    #[test]
    fn test_highlight_markdown() {
        let source = r#"# Hello

This is code"#;
        let result = highlight(source, "markdown");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-punctuation.special"));
    }

    #[test]
    fn test_highlight_c() {
        let source = r#"int main() {
    return 0;
}"#;
        let result = highlight(source, "c");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-type"));
        assert!(result.contains("ts-number"));
    }

    fn test_highlight_cpp() {
        let source = r#"#include <iostream>
int main() {
    std::cout << "hello";
}"#;
        let result = highlight(source, "cpp");
        assert!(result.contains("<span class="));
    }

    fn test_highlight_java() {
        let source = r#"class Hello {
    public static void main(String[] args) {
        System.out.println("hi");
    }
}"#;
        let result = highlight(source, "java");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-string"));
    }

    fn test_highlight_json() {
        let source = r#"{"key": "value", "num": 42}"#;
        let result = highlight(source, "json");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-string"));
        assert!(result.contains("ts-number"));
    }

    fn test_highlight_yaml() {
        let source = r#"name: hello
version: 1
enabled: true"#;
        let result = highlight(source, "yaml");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-boolean"));
    }

    fn test_highlight_toml() {
        let source = r#"[package]
name = "hello"
version = "1.0""#;
        let result = highlight(source, "toml");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-string"));
        assert!(result.contains("ts-property"));
    }

    fn test_highlight_ruby() {
        let source = r#"def hello
    puts "world"
end"#;
        let result = highlight(source, "ruby");
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-string"));
    }

    fn test_highlight_unknown_lang() {
        let result = highlight("x = 1", "unknown");
        assert!(!result.contains("<span"));
        assert_eq!(result, "x = 1");
    }
}
