# norgolith-tree-sitter-highlight

Tree-sitter syntax highlighting plugin for [Norgolith](https://github.com/NTBBloodbath/norgolith).

**Experimental.** This plugin is in early development. APIs and behavior may change without notice.

## What it does

Adds syntax highlighting to fenced code blocks in Norg pages. Supports 14 languages.

The plugin uses [tree-sitter](https://tree-sitter.github.io/) for parsing and [tree-sitter-highlight](https://crates.io/crates/tree-sitter-highlight) for token classification. Each language uses the official highlight queries bundled with its grammar crate.

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

The plugin lives in a separate repo from the Norgolith monorepo.

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
- C (`c`)
- C++ (`c++`, `cpp`, `cxx`)
- Java (`java`)
- JSON (`json`)
- YAML (`yaml`, `yml`)
- TOML (`toml`)
- Ruby (`ruby`, `rb`)
- Elixir / Erlang (`elixir`, `ex`, `exs`)
- Nix (`nix`)
- Markdown (`markdown`, `md`)

Unlisted languages fall back to plain text (no highlighting, no errors).

## Theme

The default theme uses [Catppuccin Mocha](https://github.com/catppuccin/catppuccin) colors. To customize, edit `assets/tree-sitter-theme.css` in your site directory. The file is only written once; subsequent builds will not overwrite it.

Highlight classes use the `ts-` prefix with semantic names. See `theme.css` for the full list.

## License

GPL-2.0
