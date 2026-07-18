use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use serde::Deserialize;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};
use regex::Regex;

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct PluginConfig {
    #[serde(default)]
    pub line_numbers: bool,
    #[serde(default = "default_line_start")]
    pub line_numbers_start: u32,
    #[serde(default)]
    pub css_path: Option<String>,
}

fn default_line_start() -> u32 {
    1
}

/// Cached compiled HighlightConfigurations -- avoids re-parsing queries per call.
static HL_CACHE: LazyLock<Mutex<HashMap<String, &'static HighlightConfiguration>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn cached_hl_config(lang: &str) -> Option<&'static HighlightConfiguration> {
    let mut cache = HL_CACHE.lock().unwrap();
    if let Some(cfg) = cache.get(lang) {
        return Some(cfg);
    }
    let lc = lang_config(lang)?;
    let mut cfg = Box::new(match HighlightConfiguration::new(lc.language, lang, lc.query, lc.injection, "") {
        Ok(c) => c,
        Err(_) => return None,
    });
    cfg.configure(HIGHLIGHT_NAMES);
    let cfg: &'static HighlightConfiguration = Box::leak(cfg);
    cache.insert(lang.to_string(), cfg);
    Some(cfg)
}

/// Recognized highlight names -- must match captures in .scm files.
const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "attribute.builtin",
    "boolean",
    "charset",
    "import",
    "keyframes",
    "media",
    "namespace",
    "supports",
    "character",
    "character.special",
    "comment",
    "comment.documentation",
    "comment.error",
    "comment.note",
    "comment.todo",
    "comment.warning",
    "constant",
    "constant.builtin",
    "constant.macro",
    "constructor",
    "diff.minus",
    "diff.plus",
    "function",
    "function.builtin",
    "function.call",
    "function.macro",
    "function.method",
    "function.method.call",
    "keyword",
    "keyword.conditional",
    "keyword.conditional.ternary",
    "keyword.coroutine",
    "keyword.debug",
    "keyword.directive",
    "keyword.directive.define",
    "keyword.exception",
    "keyword.function",
    "keyword.import",
    "keyword.modifier",
    "keyword.operator",
    "keyword.repeat",
    "keyword.return",
    "keyword.type",
    "label",
    "markup.heading",
    "markup.heading.1",
    "markup.heading.2",
    "markup.heading.3",
    "markup.heading.4",
    "markup.heading.5",
    "markup.heading.6",
    "markup.italic",
    "markup.link",
    "markup.link.label",
    "markup.link.url",
    "markup.list",
    "markup.list.checked",
    "markup.list.unchecked",
    "markup.quote",
    "markup.raw",
    "markup.raw.block",
    "markup.strikethrough",
    "markup.strong",
    "markup.underline",
    "module",
    "module.builtin",
    "namespace",
    "number",
    "number.float",
    "operator",
    "property",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.documentation",
    "string.escape",
    "string.regexp",
    "string.special",
    "string.special.key",
    "string.special.path",
    "string.special.symbol",
    "string.special.url",
    "tag",
    "tag.attribute",
    "tag.builtin",
    "tag.delimiter",
    "type",
    "type.builtin",
    "type.definition",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
    "variable.parameter.builtin",
];

#[derive(Clone)]
struct LanguageConfig {
    language: tree_sitter::Language,
    query: &'static str,
    injection: &'static str,
}

fn lang_config(lang: &str) -> Option<LanguageConfig> {
    match lang {
        "rust" | "rs" => Some(LanguageConfig {
            language: tree_sitter_rust::LANGUAGE.into(),
            query: include_str!("queries/rust.scm"),
            injection: include_str!("queries/line_block_comment_injection.scm"),
        }),
        "python" | "py" => Some(LanguageConfig {
            language: tree_sitter_python::LANGUAGE.into(),
            query: include_str!("queries/python.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "javascript" | "js" | "jsx" => Some(LanguageConfig {
            language: tree_sitter_javascript::LANGUAGE.into(),
            query: include_str!("queries/javascript.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "html" => Some(LanguageConfig {
            language: tree_sitter_html::LANGUAGE.into(),
            query: include_str!("queries/html.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "css" => Some(LanguageConfig {
            language: tree_sitter_css::LANGUAGE.into(),
            query: include_str!("queries/css.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "bash" | "sh" | "shell" => Some(LanguageConfig {
            language: tree_sitter_bash::LANGUAGE.into(),
            query: include_str!("queries/bash.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "nix" => Some(LanguageConfig {
            language: tree_sitter_nix::LANGUAGE.into(),
            query: include_str!("queries/nix.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "elixir" | "ex" | "exs" => Some(LanguageConfig {
            language: tree_sitter_elixir::LANGUAGE.into(),
            query: include_str!("queries/elixir.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "typescript" | "ts" => Some(LanguageConfig {
            language: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            query: include_str!("queries/typescript.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "markdown" | "md" => Some(LanguageConfig {
            language: tree_sitter_md::LANGUAGE.into(),
            query: include_str!("queries/markdown.scm"),
            injection: "",
        }),
        "c" => Some(LanguageConfig {
            language: tree_sitter_c::LANGUAGE.into(),
            query: include_str!("queries/c.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "c++" | "cpp" | "cxx" => Some(LanguageConfig {
            language: tree_sitter_cpp::LANGUAGE.into(),
            query: include_str!("queries/cpp.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "java" => Some(LanguageConfig {
            language: tree_sitter_java::LANGUAGE.into(),
            query: include_str!("queries/java.scm"),
            injection: include_str!("queries/line_block_comment_injection.scm"),
        }),
        "json" => Some(LanguageConfig {
            language: tree_sitter_json::LANGUAGE.into(),
            query: include_str!("queries/json.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "yaml" | "yml" => Some(LanguageConfig {
            language: tree_sitter_yaml::LANGUAGE.into(),
            query: include_str!("queries/yaml.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "toml" => Some(LanguageConfig {
            language: tree_sitter_toml_updated::language().into(),
            query: include_str!("queries/toml.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "ruby" | "rb" => Some(LanguageConfig {
            language: tree_sitter_ruby::LANGUAGE.into(),
            query: include_str!("queries/ruby.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "go" | "golang" => Some(LanguageConfig {
            language: tree_sitter_go::LANGUAGE.into(),
            query: include_str!("queries/go.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "make" | "makefile" | "mk" => Some(LanguageConfig {
            language: tree_sitter_make::LANGUAGE.into(),
            query: include_str!("queries/make.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "lua" => Some(LanguageConfig {
            language: tree_sitter_lua::LANGUAGE.into(),
            query: include_str!("queries/lua.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "php" => Some(LanguageConfig {
            language: tree_sitter_php::LANGUAGE_PHP_ONLY.into(),
            query: include_str!("queries/php.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "sql" | "postgres" | "psql" | "sequel" => Some(LanguageConfig {
            language: tree_sitter_sequel::LANGUAGE.into(),
            query: include_str!("queries/sql.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "docker" | "dockerfile" | "Dockerfile" | "containerfile" => Some(LanguageConfig {
            language: tree_sitter_containerfile::LANGUAGE.into(),
            query: include_str!("queries/dockerfile.scm"),
            injection: include_str!("queries/generic_comment_injection.scm"),
        }),
        "tera" | "tpl" => Some(LanguageConfig {
            language: tree_sitter_tera::LANGUAGE.into(),
            query: include_str!("queries/tera.scm"),
            injection: include_str!("queries/tera_injections.scm"),
        }),
        "diff" => Some(LanguageConfig {
            language: tree_sitter_diff::LANGUAGE.into(),
            query: include_str!("queries/diff.scm"),
            injection: "",
        }),
        "comment" => Some(LanguageConfig {
            language: tree_sitter_comment::LANGUAGE.into(),
            query: include_str!("queries/comment.scm"),
            injection: "",
        }),
        _ => None,
    }
}

/// Highlight source code using tree-sitter for the given language.
///
/// Returns HTML with `<span class="ts-{name}">` wrapped tokens.
/// Falls back to plain text if the language is unsupported or parsing fails.
pub fn highlight(source: &str, lang: &str, config: &PluginConfig) -> String {
    let is_markdown = matches!(lang, "markdown" | "md");

    let mut html = if is_markdown {
        markdown_highlight(source, lang)
    } else {
        match cached_hl_config(lang) {
            Some(cfg) => render(cfg, source),
            None => html_escape(source),
        }
    };

    if config.line_numbers {
        html = add_line_numbers(&html, config.line_numbers_start);
    }

    html
}

fn markdown_highlight(source: &str, lang: &str) -> String {
    let lc = match lang_config(lang) {
        Some(lc) => lc,
        None => return html_escape(source),
    };

    let inline_query = include_str!("queries/markdown_inline.scm");

    let md_injection = include_str!("queries/markdown_injections.scm");

    let mut md_config = match HighlightConfiguration::new(
        tree_sitter_md::LANGUAGE.into(),
        lang,
        lc.query,
        &md_injection,
        "",
    ) {
        Ok(c) => c,
        Err(_) => return html_escape(source),
    };
    md_config.configure(HIGHLIGHT_NAMES);

    let mut inline_config = match HighlightConfiguration::new(
        tree_sitter_md::INLINE_LANGUAGE.into(),
        "markdown_inline",
        inline_query,
        "",
        "",
    ) {
        Ok(c) => c,
        Err(_) => return render(&md_config, source),
    };
    inline_config.configure(HIGHLIGHT_NAMES);

    let mut highlighter = Highlighter::new();
    let events = match highlighter.highlight(
        &md_config,
        source.as_bytes(),
        None,
        |name| {
            if name == "markdown_inline" {
                Some(&inline_config)
            } else {
                cached_hl_config(name)
            }
        },
    ) {
        Ok(e) => e,
        Err(_) => return html_escape(source),
    };
    let mut renderer = HtmlRenderer::new();
    let _ = renderer.render(events, source.as_bytes(), &|highlight, out| {
        emit_capture(highlight.0, out);
    });
    let mut html = String::from_utf8_lossy(&renderer.html).to_string();

    // _block_close (external scanner token) extends fenced_code_block past
    // the closing delimiter, consuming trailing blank lines. HtmlRenderer
    // splits at \n and reopens the @markup.raw.block span on the blank line,
    // giving it code-block background. Remove the span wrapper but keep the
    // blank line by replacing \n + empty_span + \n with \n\n.
    // Internal empty lines are unaffected (preceding span isn't backtick-only).
    // trim_end() then strips any trailing \n\n, so EOF blank lines disappear.
    let re = Regex::new(
        r#"(<span class="ts-markup ts-raw ts-block">`+</span>)\n<span class="ts-markup ts-raw ts-block"></span>\n"#,
    )
    .unwrap();
    html = re.replace_all(&html, "$1\n\n").to_string();

    html.trim_end().to_string()
}

fn add_line_numbers(html: &str, start: u32) -> String {
    let mut result = String::with_capacity(html.len() + 32);
    let mut line_num = start;
    for line in html.split('\n') {
        if line_num > start {
            result.push('\n');
        }
        if line.is_empty() {
            result.push_str("<span class=\"ts-line\" data-ln=\"");
            result.push_str(&line_num.to_string());
            result.push_str("\"></span>");
        } else {
            result.push_str("<span class=\"ts-line\" data-ln=\"");
            result.push_str(&line_num.to_string());
            result.push_str("\">");
            result.push_str(line);
            result.push_str("</span>");
        }
        line_num += 1;
    }
    result
}

fn render(config: &HighlightConfiguration, source: &str) -> String {
    let mut highlighter = Highlighter::new();

    let events = match highlighter.highlight(config, source.as_bytes(), None, |name| {
        cached_hl_config(name)
    }) {
        Ok(e) => e,
        Err(_) => return html_escape(source),
    };
    let mut renderer = HtmlRenderer::new();
    let _ = renderer.render(events, source.as_bytes(), &|highlight, out| {
        emit_capture(highlight.0, out);
    });
    String::from_utf8_lossy(&renderer.html).trim_end().to_string()
}

fn emit_capture(idx: usize, out: &mut Vec<u8>) {
    if let Some(name) = HIGHLIGHT_NAMES.get(idx) {
        out.extend_from_slice(b"class=\"");
        let mut first = true;
        for part in name.split('.') {
            if !first {
                out.extend_from_slice(b" ");
            }
            out.extend_from_slice(b"ts-");
            out.extend_from_slice(part.as_bytes());
            first = false;
        }
        out.extend_from_slice(b"\"");
    }
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

    fn default_cfg() -> PluginConfig {
        PluginConfig::default()
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("fn main() {}"), "fn main() {}");
        assert_eq!(html_escape("<div>"), "&lt;div&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
    }

    #[test]
    fn test_highlight_comment() {
        let source = r#"// this is a comment
fn main() {}"#;
        let result = highlight(source, "rust", &default_cfg());
        assert!(
            result.contains("ts-comment"),
            "expected ts-comment in: {result}"
        );
    }

    #[test]
    fn test_highlight_rust() {
        let source = r#"fn main() {
    println!("hello");
}"#;
        let result = highlight(source, "rust", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-function"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_python() {
        let source = r#"def hello():
    print("world")"#;
        let result = highlight(source, "python", &default_cfg());
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
        let result = highlight(source, "javascript", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-function"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_javascript_template() {
        let source = r#"return `Hello, ${name}!`;"#;
        let result = highlight(source, "javascript", &default_cfg());
        assert!(
            result.contains("ts-punctuation ts-special"),
            "expected ts-punctuation ts-special in: {result}"
        );
    }

    #[test]
    fn test_highlight_html() {
        let source = r#"<div class="foo">hello</div>"#;
        let result = highlight(source, "html", &default_cfg());
        assert!(result.contains("<span class="), "html: {result}");
        assert!(result.contains("ts-tag"));
        assert!(result.contains("ts-attribute"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_css() {
        let source = r#"body { color: red; }"#;
        let result = highlight(source, "css", &default_cfg());
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
        let result = highlight(source, "bash", &default_cfg());
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
        let result = highlight(source, "nix", &default_cfg());
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
        let result = highlight(source, "elixir", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_typescript() {
        let source = r#"interface Foo {
    name: string;
}"#;
        let result = highlight(source, "typescript", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-type"));
    }

    #[test]
    fn test_highlight_markdown() {
        let source = r#"# Hello

This is code"#;
        let result = highlight(source, "markdown", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-markup ts-heading ts-1"));
    }

    #[test]
    fn test_highlight_markdown_codeblock() {
        let source = r#"# Hello

This is **bold** and _italic_.

- Item 1
- Item 2

```lua
local function greet(name)
  print("Hello " .. name .. "!")

end
```

```python
# Greet someone
def greet(name: str) -> str:
    return f"Hello, {name}!"

print(greet("world"))
```"#;
        let result = highlight(source, "markdown", &default_cfg());
        println!("=== MARKDOWN CODEBLOCK ===");
        println!("{}", result);
        println!("=== END ===");
        assert!(result.contains("ts-keyword"), "has keyword: {result}");
        assert!(result.contains("ts-function"), "has function: {result}");
        assert!(result.contains("ts-string"), "has string: {result}");
        // no empty punctuation.special spans in code
        let n = result.matches("<span class=\"ts-punctuation ts-special\"></span>").count();
        assert_eq!(n, 0, "no empty punctuation spans, found: {n}");
    }

    #[test]
    fn test_highlight_markdown_codeblock_simple() {
        let source = r#"```lua
local x = 1
```"#;
        let result = highlight(source, "markdown", &default_cfg());
        assert!(result.contains("ts-keyword"), "has keyword: {result}");
        assert!(result.contains("ts-number"), "has number: {result}");
    }

    #[test]
    fn test_highlight_markdown_inline() {
        let source = "Hello **bold** and `code`";
        let result = highlight(source, "markdown", &default_cfg());
        assert!(
            result.contains("ts-markup ts-strong"),
            "expected ts-markup ts-strong in result: {result}"
        );
        assert!(
            result.contains("ts-markup ts-raw"),
            "expected ts-markup ts-raw in result: {result}"
        );
    }

    #[test]
    fn test_highlight_c() {
        let source = r#"int main() {
    return 0;
}"#;
        let result = highlight(source, "c", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-type"));
        assert!(result.contains("ts-number"));
    }

    #[test]
    fn test_highlight_cpp() {
        let source = r#"auto x = nullptr;

class Foo {
public:
    void bar() {}
};

template<typename T>
T add(T a, T b) { return a + b; }
"#;
        let result = highlight(source, "cpp", &default_cfg());
        assert!(result.contains("<span class="), "cpp hl:\n{result}");
    }

    #[test]
    fn test_highlight_java() {
        let source = r#"class Hello {
    public static void main(String[] args) {
        System.out.println("hi");
    }
}"#;
        let result = highlight(source, "java", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_json() {
        let source = r#"{"key": "value", "num": 42}"#;
        let result = highlight(source, "json", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-string"));
        assert!(result.contains("ts-number"));
    }

    #[test]
    fn test_highlight_yaml() {
        let source = r#"name: hello
version: 1
enabled: true"#;
        let result = highlight(source, "yaml", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-boolean"));
    }

    #[test]
    fn test_highlight_toml() {
        let source = r#"[package]
name = "hello"
version = "1.0""#;
        let result = highlight(source, "toml", &default_cfg());
        assert!(result.contains("<span class="));
        assert!(result.contains("ts-string"));
        assert!(result.contains("ts-property"));
    }

    #[test]
    fn test_highlight_ruby() {
        let source = r#"def hello
    puts "world"
end"#;
        let result = highlight(source, "ruby", &default_cfg());
        assert!(result.contains("<span class="), "ruby output: {result}");
    }

    #[test]
    fn test_highlight_go() {
        let source = r#"func main() {
    fmt.Println("hello")
}"#;
        let result = highlight(source, "go", &default_cfg());
        assert!(result.contains("<span class="), "go: {result}");
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-function"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_lua() {
        let source = r#"function hello()
    print("world")
end"#;
        let result = highlight(source, "lua", &default_cfg());
        assert!(result.contains("<span class="), "lua: {result}");
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_lua_local_function() {
        let source = r#"local function greet(name)
  print("Hello " .. name .. "!")
end"#;
        let result = highlight(source, "lua", &default_cfg());
        assert!(result.contains("ts-function"), "ts-function: {result}");
        assert!(result.contains("ts-keyword"), "ts-keyword: {result}");
        assert!(result.contains("ts-string"), "ts-string: {result}");
    }



    #[test]
    fn test_highlight_php() {
        let source = r#"<?php
function hello() {
    echo "world";
}"#;
        let result = highlight(source, "php", &default_cfg());
        assert!(result.contains("<span class="), "php: {result}");
        assert!(result.contains("ts-keyword"));
        assert!(result.contains("ts-function"));
        assert!(result.contains("ts-string"));
    }

    #[test]
    fn test_highlight_sql() {
        let source = "SELECT * FROM users WHERE id = 1; -- active users";
        let result = highlight(source, "sql", &default_cfg());
        assert!(result.contains("<span class="), "sql: {result}");
        assert!(result.contains("ts-keyword"), "sql keyword: {result}");
        assert!(result.contains("ts-comment"), "sql comment: {result}");
    }

    #[test]
    fn test_highlight_diff() {
        let source = r#"diff --git a/foo b/foo
index abc..def 100644
--- a/foo
+++ b/foo
@@ -1 +1 @@
-hello
+world
"#;
        let result = highlight(source, "diff", &default_cfg());
        assert!(result.contains("ts-diff ts-plus"), "diff plus: {result}");
        assert!(result.contains("ts-diff ts-minus"), "diff minus: {result}");
    }

    #[test]
    fn test_highlight_comment_tags() {
        let source = "TODO: this needs work\nFIXME(goofy): crash here";
        let result = highlight(source, "comment", &default_cfg());
        assert!(
            result.contains("ts-comment ts-todo"),
            "expected ts-comment ts-todo in: {result}"
        );
        assert!(
            result.contains("ts-comment ts-error"),
            "expected ts-comment ts-error in: {result}"
        );
    }

    #[test]
    fn test_highlight_make() {
        let source = "all:\n\t@echo \"hello\"\n\n# comment";
        let result = highlight(source, "make", &default_cfg());
        assert!(result.contains("<span class="), "make: {result}");
        assert!(result.contains("ts-function"), "make function: {result}");
        assert!(result.contains("ts-comment"), "make comment: {result}");
    }

    #[test]
    fn test_highlight_dockerfile() {
        let source = "FROM ubuntu:22.04\nRUN apt update\n# comment";
        let result = highlight(source, "dockerfile", &default_cfg());
        assert!(result.contains("<span class="), "docker: {result}");
        assert!(result.contains("ts-keyword"), "docker keyword: {result}");
        assert!(result.contains("ts-comment"), "docker comment: {result}");
    }

    #[test]
    fn test_highlight_tera() {
        let source = "{% if x %}{{ y }}{% endif %}";
        let result = highlight(source, "tera", &default_cfg());
        assert!(result.contains("<span class="), "tera: {result}");
        assert!(result.contains("ts-keyword"), "tera keyword: {result}");
    }

    #[test]
    fn test_highlight_tera_html() {
        let source = "<div class=\"foo\">{{ name }}</div>";
        let result = highlight(source, "tera", &default_cfg());
        assert!(result.contains("ts-tag"), "tera html tag: {result}");
        assert!(result.contains("ts-string"), "tera html string: {result}");
        assert!(result.contains("ts-variable"), "tera variable: {result}");
        assert!(result.contains("</span><span class=\"ts-tag\">div</span>"), "closing tag name: {result}");
    }

    #[test]
    fn test_highlight_unknown_lang() {
        let result = highlight("x = 1", "unknown", &default_cfg());
        assert!(!result.contains("<span"));
        assert_eq!(result, "x = 1");
    }

    #[test]
    fn test_line_numbers() {
        let cfg = PluginConfig {
            line_numbers: true,
            line_numbers_start: 1,
        };
        let source = "x = 1";
        let result = highlight(source, "python", &cfg);
        assert!(result.contains("ts-line"), "ln: {result}");
        assert!(result.contains("data-ln=\"1\""), "ln: {result}");
    }
}
