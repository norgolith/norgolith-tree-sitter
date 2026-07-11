# norgolith-tree-sitter-highlight

Tree-sitter syntax highlighting plugin for [Norgolith](https://github.com/NTBBloodbath/norgolith).

**Experimental.** This plugin is in early development. APIs and behavior may change without notice.

## What it does

Adds syntax highlighting to fenced code blocks in Norg pages. Supports 25 languages.

The plugin uses [tree-sitter](https://tree-sitter.github.io/) for parsing and [tree-sitter-highlight](https://crates.io/crates/tree-sitter-highlight) for token classification. Highlight queries come from [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) -- the same queries Neovim uses, adapted for Rust tree-sitter (predicate conversion, `; inherits:` resolution).

## How it works

1. `post_convert` hook: extracts `<pre><code class="language-X">` blocks from the HTML fragment, highlights them with tree-sitter, and writes the result back.
2. `post_render` hook: injects a `<link>` to the theme CSS in the final HTML if any highlighted code is present.
3. On first run, the plugin writes `assets/tree-sitter-theme.css` with a default dark theme (Catppuccin Mocha).

## Installation

From a Norgolith site directory:

```sh
lith plugin install /path/to/norgolith-tree-sitter-highlight
```

Or manually, copy the `.so` and `plugin.toml` into `plugins/norgolith-tree-sitter-highlight/`.

## Development

```sh
cargo build --release
cp target/release/libnorgolith_tree_sitter_highlight.so \
   /path/to/my-site/plugins/norgolith-tree-sitter-highlight/
rm -f /path/to/my-site/assets/tree-sitter-theme.css
cargo run -- build   # from the site directory
```

## Hooks

| Hook | Runs when | What it does |
|------|-----------|--------------|
| `post_convert` | After Norg to HTML conversion | Highlights code blocks in the HTML fragment |
| `post_render` | After template rendering | Injects the CSS link into `<head>` |

## Supported languages

- Rust (`rust`, `rs`)
- Python (`python`, `py`)
- JavaScript (`javascript`, `js`, `jsx`)
- TypeScript (`typescript`, `ts`)
- HTML (`html`)
- CSS (`css`)
- Bash / Shell (`bash`, `sh`, `shell`)
- Nix (`nix`)
- Elixir / Erlang (`elixir`, `ex`, `exs`)
- Markdown (`markdown`, `md`)
- C (`c`)
- C++ (`c++`, `cpp`, `cxx`)
- Java (`java`)
- JSON (`json`)
- YAML (`yaml`, `yml`)
- TOML (`toml`)
- Ruby (`ruby`, `rb`)
- Go (`go`, `golang`)
- Lua (`lua`)
- PHP (`php`)
- SQL (`sql`, `postgres`, `psql`, `sequel`)
- Make (`make`, `makefile`, `mk`)
- Docker / Containerfile (`docker`, `dockerfile`, `containerfile`)
- Tera (`tera`, `tpl`)
- DIFF (`diff`)

Unlisted languages fall back to plain text (no highlighting, no errors).

## Theme

The default theme uses [Catppuccin Mocha](https://github.com/catppuccin/catppuccin) colors with 96 semantic capture groups covering keywords, types, functions, variables, markup, tags, punctuation, operators, and CSS at-rules. Highlight classes use the `ts-` prefix with dot-separated sub-classes (e.g., `class="ts-keyword ts-conditional"`).

To customize, edit `assets/tree-sitter-theme.css` in your site directory. The file is only written once; subsequent builds will not overwrite it.

## Configuration

Add to your site's `norgolith.toml`:

```toml
[plugins.norgolith-tree-sitter-highlight]
line-numbers = true
line-numbers-start = 1
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `line-numbers` | bool | `false` | Add line numbers to code blocks |
| `line-numbers-start` | integer | `1` | Starting line number |

## License

GPL-2.0
