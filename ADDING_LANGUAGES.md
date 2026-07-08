# Adding New Languages

Guide for contributors who want to add support for a new programming language.

## How It Works

The plugin has three stages:

1. **HTML parsing** (`highlight.rs`): finds `<pre><code class="language-X">` blocks in HTML
2. **Tree-sitter highlighting** (`ts_highlight.rs`): tokenizes source code with a grammar and query file
3. **CSS injection** (`lib.rs`): writes `theme.css` to the site's `assets/` dir and injects `<link>` in `<head>`

Each language needs three things: a grammar crate, a highlight query file, and a match arm in `lang_config()`.

## Step-by-Step

### 1. Add the grammar crate

Find the tree-sitter grammar crate on crates.io (e.g., `tree-sitter-go`, `tree-sitter-java`). Add it to `Cargo.toml`:

```toml
# Cargo.toml
tree-sitter-go = "~0.23"
```

Check the crate's docs for the language constant name (usually `LANGUAGE`).

### 2. Create the highlight query

Create `src/queries/<language>.scm`. This file maps tree-sitter node types to highlight names.

```scheme
; src/queries/go.scm

; Keywords
"func" @keyword
"return" @keyword
"if" @keyword
"else" @keyword
"for" @keyword

; Functions
(call_expression
  function: (identifier) @function)

(function_declaration
  name: (identifier) @function)

; Strings
(interpreted_string_literal) @string
(raw_string_literal) @string

; Comments
(comment) @comment

; Types
(type_identifier) @type
(struct_type) @type

; Numbers
(int_literal) @number
(float_literal) @number
```

**Finding node types**: Run `tree-sitter parse` on example code to see the AST, or check the grammar's `src/node-types.json`.

**Available highlight names** (must be one of these):

| Name | What it matches |
|------|----------------|
| `keyword` | `if`, `for`, `return`, `fn`, etc. |
| `function` | function/method calls and definitions |
| `string` | string literals |
| `number` | integer and float literals |
| `comment` | comments |
| `type` | type names |
| `variable` | variable references |
| `operator` | `+`, `-`, `=`, etc. |
| `property` | struct fields, object properties |
| `constant` | constants, ALL_CAPS names |
| `boolean` | `true`, `false` |
| `tag` | HTML tag names |
| `attribute` | HTML attributes, decorators |
| `constructor` | enum variants, constructors |
| `punctuation.bracket` | `()`, `{}`, `[]` |
| `punctuation.delimiter` | `;`, `,`, `.` |

### 3. Add the match arm

Edit `src/ts_highlight.rs`, add a new arm to `lang_config()`:

```rust
fn lang_config(lang: &str) -> Option<LanguageConfig> {
    match lang {
        // ... existing languages ...

        "go" => Some(LanguageConfig {
            language: tree_sitter_go::LANGUAGE.into(),
            query: include_str!("queries/go.scm"),
        }),
        _ => None,
    }
}
```

The string before `|` is what users write in their code blocks (`@code go`). Add aliases if the language has common short names:

```rust
"rust" | "rs" => ...
"python" | "py" => ...
"javascript" | "js" | "jsx" => ...
"go" | "golang" => ...
```

### 4. Add CSS for new highlight names (if any)

If your query uses highlight names not yet in `theme.css`, add CSS rules:

```css
/* theme.css */

/* New language-specific classes */
.ts-keyword {
    color: #cba6f7;
    font-weight: bold;
}
```

If the names are already covered by existing rules (`.ts-keyword`, `.ts-function`, etc.), skip this step.

### 5. Add a test

Add a test in `src/ts_highlight.rs`:

```rust
#[test]
fn test_highlight_go() {
    let source = r#"func main() {
    fmt.Println("hello")
}"#;
    let result = highlight(source, "go");
    assert!(result.contains("<span class="));
    assert!(result.contains("ts-keyword"));
    assert!(result.contains("ts-function"));
    assert!(result.contains("ts-string"));
}
```

## Testing

```sh
# Build the plugin
cargo build --release

# Copy to your test site
cp target/release/libnorgolith_tree_sitter_highlight.so \
   /path/to/my-site/plugins/norgolith-tree-sitter-highlight/

# Build the site (from site directory)
cd /path/to/my-site
lith build
```

Check the generated HTML in `public/` — code blocks should have `<span class="ts-*">` wrapped tokens.

## File Reference

```
src/
├── lib.rs              # Plugin hooks (post_convert, post_render)
├── highlight.rs        # HTML regex extraction, entity decoding
├── ts_highlight.rs     # Tree-sitter highlighting + lang_config()
└── queries/
    ├── rust.scm
    ├── python.scm
    ├── javascript.scm
    ├── html.scm
    └── css.scm
theme.css               # Default dark theme (Catppuccin Mocha)
```
