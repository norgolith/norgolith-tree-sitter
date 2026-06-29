mod highlight;
mod ts_highlight;

use norgolith_plugin_sdk::*;

/// Embed the default theme CSS
const DEFAULT_THEME: &str = include_str!("../theme.css");

/// Post-convert handler: highlight code blocks in the HTML fragment.
fn post_convert_handler(json: serde_json::Value) -> Result<Option<String>, String> {
    let ctx: TransformContext = serde_json::from_value(json).map_err(|e| e.to_string())?;
    let highlighted = highlight::highlight_codeblocks(&ctx.html);
    ensure_theme_css();
    if highlighted == ctx.html {
        Ok(None)
    } else {
        Ok(Some(highlighted))
    }
}

/// Post-render handler: inject the tree-sitter theme CSS into the final HTML.
fn post_render_handler(json: serde_json::Value) -> Result<Option<String>, String> {
    let ctx: TransformContext = serde_json::from_value(json).map_err(|e| e.to_string())?;
    let html = &ctx.html;

    // Only inject if we actually highlighted something (has ts- classes)
    if !html.contains("ts-") {
        return Ok(None);
    }

    let css_tag = r#"<link rel="stylesheet" href="/assets/tree-sitter-theme.css" />"#;

    // Inject before </head>
    if let Some(pos) = html.find("</head>") {
        let mut result = String::with_capacity(html.len() + css_tag.len() + 1);
        result.push_str(&html[..pos]);
        result.push('\n');
        result.push_str(css_tag);
        result.push('\n');
        result.push_str(&html[pos..]);
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

/// Write the default theme CSS to the site directory if it doesn't exist.
fn ensure_theme_css() {
    let Ok(cwd) = std::env::current_dir() else {
        return;
    };

    let theme_path = cwd.join("assets").join("tree-sitter-theme.css");
    if theme_path.exists() {
        return;
    }

    let _ = std::fs::create_dir_all(cwd.join("assets"));

    if let Err(e) = std::fs::write(&theme_path, DEFAULT_THEME) {
        eprintln!("[norgolith-tree-sitter-highlight] Failed to write theme CSS: {}", e);
    }
}

register_plugin!("norgolith-tree-sitter-highlight", "0.1.0",
    hooks: [post_convert: post_convert_handler, post_render: post_render_handler]
);
