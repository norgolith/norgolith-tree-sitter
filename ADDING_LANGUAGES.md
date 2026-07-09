# Adding New Languages

Guide for contributors who want to add support for a new programming language.

## How It Works

The plugin has three stages:

1. **HTML parsing** (`highlight.rs`): finds `<pre><code class="language-X">` blocks in HTML
2. **Tree-sitter highlighting** (`ts_highlight.rs`): tokenizes source code with a grammar and query file
3. **CSS injection** (`lib.rs`): writes `theme.css` to the site's `assets/` dir and injects `<link>` in `<head>`

Each language needs three things: a grammar crate, a highlight query file in `src/queries/`, and a match arm in `lang_config()`.

## Step-by-Step

### 1. Add the grammar crate

Find the tree-sitter grammar crate on crates.io (e.g., `tree-sitter-go`). Add it to `Cargo.toml` with the latest version:

```toml
tree-sitter-go = "0.25.0"
```

The crate must export `language()` or `LANGUAGE` (the tree-sitter `Language`). Some crates expose multiple grammars — e.g., `tree-sitter-php` has `LANGUAGE_PHP` (full PHP with HTML) and `LANGUAGE_PHP_ONLY` (pure PHP). Use the one appropriate for code blocks.

### 2. Create the highlight query file

Create `src/queries/<language>.scm`. This file maps tree-sitter AST node types to highlight names using the same capture names as **nvim-treesitter** (`@keyword`, `@function.call`, `@variable`, etc.).

Start from the [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/tree/master/queries) query for your language:

```scheme
; src/queries/go.scm

; Keywords
"func" @keyword.function
"return" @keyword.return
"if" @keyword.conditional
"else" @keyword.conditional
"for" @keyword.repeat

; Functions
(call_expression
  function: (identifier) @function.call)

(function_declaration
  name: (identifier) @function)

; Strings
(interpreted_string_literal) @string
(raw_string_literal) @string

; Comments
(comment) @comment

; Types
(type_identifier) @type

; Numbers
(int_literal) @number
(float_literal) @number

; Operators
"+" @operator
"-" @operator
"=" @operator

; Punctuation
"," @punctuation.delimiter
";" @punctuation.delimiter
"(" @punctuation.bracket
")" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
```

**Finding node types**: Run `tree-sitter parse` on example code to see the AST, or check the grammar's `src/node-types.json`.

**; inherits:** — if the query file contains `; inherits: <base>`, resolve it at copy time by concatenating the base file before the language-specific rules (the plugin does not resolve these at runtime).

**#lua-match? → #match?** — nvim-treesitter uses Neovim Lua predicates (`#lua-match?`). Rust tree-sitter only supports `#match?`, `#eq?`, `#any-of?`, `#not-match?`, `#not-eq?`, `#any-not-of?`. Convert Lua patterns to regex:
- `%d` → `[0-9]`, `%u` → `[A-Z]`, `%l` → `[a-z]`, `%w` → `[0-9A-Za-z]`
- `%a` → `[A-Za-z]`, `%s` → `[ \t\n]`
- Escape sequences: `%.` → `\\.`, `%$` → `\\$`

### 3. Add the match arm

Edit `src/ts_highlight.rs`. Add the query as `include_str!` in `lang_config()`:

```rust
fn lang_config(lang: &str) -> Option<LanguageConfig> {
    match lang {
        // ... existing languages ...

        "go" | "golang" => Some(LanguageConfig {
            language: tree_sitter_go::LANGUAGE.into(),
            query: include_str!("queries/go.scm"),
        }),
        // Crate with multiple grammars:
        "php" => Some(LanguageConfig {
            language: tree_sitter_php::LANGUAGE_PHP_ONLY.into(),
            query: include_str!("queries/php.scm"),
        }),
        _ => None,
    }
}
```

The match result is cached automatically by `LANG_CACHE` (a `LazyLock<Mutex<HashMap<String, LanguageConfig>>>`) — the first call per language stores the config, subsequent calls skip the O(n) match.

### 4. Register highlight names (if needed)

If your query uses capture names not already in `HIGHLIGHT_NAMES`, add them to the list in `ts_highlight.rs`:

```rust
const HIGHLIGHT_NAMES: &[&str] = &[
    // ... existing names ...
    "go.test",
    "go.package",
];
```

Check the existing list first — most names from nvim-treesitter are already registered (94 names currently).

### 5. Add CSS (if needed)

If you added new capture names, add CSS rules in `theme.css`:

```css
.ts-go.ts-test { color: #a6e3a1; font-style: italic; }
.ts-go.ts-package { color: #f5c2e7; }
```

### 6. Add a test

Add a test in `src/ts_highlight.rs`. Use `PluginConfig::default()` for the third argument:

```rust
#[test]
fn test_highlight_go() {
    let source = r#"func main() {
    fmt.Println("hello")
}"#;
    let result = highlight(source, "go", &PluginConfig::default());
    assert!(result.contains("<span class="), "go: {result}");
    assert!(result.contains("ts-function"));
    assert!(result.contains("ts-string"));
}
```

## Testing

```sh
cargo test
```

## Capture Names

All 94 registered highlight names. Every name supports dot-separated sub-classes in CSS (e.g., `ts-keyword ts-conditional`).

### Type & Structure
| Capture | Category |
|---------|----------|
| `@type`, `@type.builtin`, `@type.definition` | Types |
| `@interface` | Interfaces |
| `@constructor` | Constructors |
| `@module`, `@module.builtin`, `@namespace` | Modules |

### Functions
| Capture | Category |
|---------|----------|
| `@function`, `@function.builtin`, `@function.call` | Functions |
| `@function.macro` | Macros |
| `@function.method`, `@function.method.call` | Methods |

### Variables
| Capture | Category |
|---------|----------|
| `@variable`, `@variable.builtin` | Variables |
| `@variable.member` | Fields |
| `@variable.parameter`, `@variable.parameter.builtin` | Parameters |

### Keywords
| Capture | Category |
|---------|----------|
| `@keyword`, `@keyword.conditional`, `@keyword.conditional.ternary` | Keywords |
| `@keyword.coroutine`, `@keyword.debug`, `@keyword.directive` | |
| `@keyword.directive.define`, `@keyword.exception` | |
| `@keyword.function`, `@keyword.import`, `@keyword.modifier` | |
| `@keyword.operator`, `@keyword.repeat`, `@keyword.return` | |
| `@keyword.type` | |

### Literals
| Capture | Category |
|---------|----------|
| `@string`, `@string.documentation` | Strings |
| `@string.escape`, `@string.regexp` | |
| `@string.special`, `@string.special.key`, `@string.special.path` | |
| `@string.special.symbol`, `@string.special.url` | |
| `@character`, `@character.special` | Characters |
| `@number`, `@number.float` | Numbers |
| `@boolean` | Booleans |
| `@constant`, `@constant.builtin`, `@constant.macro` | Constants |

### Markup
| Capture | Category |
|---------|----------|
| `@markup.heading`, `@markup.heading.1`–`@markup.heading.6` | Headings |
| `@markup.italic`, `@markup.strong` | Emphasis |
| `@markup.strikethrough`, `@markup.underline` | Decoration |
| `@markup.link`, `@markup.link.label`, `@markup.link.url` | Links |
| `@markup.list`, `@markup.list.checked`, `@markup.list.unchecked` | Lists |
| `@markup.quote`, `@markup.raw`, `@markup.raw.block` | Quotes / raw |

### Tags & Attributes
| Capture | Category |
|---------|----------|
| `@tag`, `@tag.attribute`, `@tag.builtin`, `@tag.delimiter` | HTML tags |
| `@tag.error` | Invalid tags |
| `@attribute`, `@attribute.builtin` | Attributes |

### Punctuation & Operators
| Capture | Category |
|---------|----------|
| `@punctuation.bracket`, `@punctuation.delimiter`, `@punctuation.special` | Punctuation |
| `@operator` | Operators |
| `@label` | Labels |

### Metadata
| Capture | Category |
|---------|----------|
| `@comment`, `@comment.documentation` | Comments |
| `@property` | Properties |
| `@escape` | Escape sequences |
| `@embedded` | Embedded content |

### CSS at-rules
| Capture | Category |
|---------|----------|
| `@charset`, `@import`, `@keyframes`, `@media`, `@supports` | At-rules |

## File Reference

```
src/
├── lib.rs              # Plugin hooks (post_convert, post_render) + config parsing
├── highlight.rs        # HTML regex extraction, entity decoding
├── ts_highlight.rs     # Tree-sitter highlighting + PluginConfig + LANG_CACHE + HIGHLIGHT_NAMES
└── queries/            # nvim-treesitter highlight queries (23 files)
    ├── rust.scm
    ├── python.scm
    ├── javascript.scm
    ├── go.scm
    ├── lua.scm
    ├── php.scm
    ├── ...
    ├── html.scm
    └── html_tags.scm
theme.css               # Default dark theme (Catppuccin Mocha)
```
