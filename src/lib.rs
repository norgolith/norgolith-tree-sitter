mod highlight;
mod ts_highlight;

use norgolith_plugin_sdk::*;
use ts_highlight::PluginConfig;

/// Embed the default theme CSS
const DEFAULT_THEME: &str = include_str!("../theme.css");

/// Post-convert handler: highlight code blocks in the HTML fragment.
fn post_convert_handler(json: serde_json::Value) -> Result<Option<String>, String> {
    let ctx: TransformContext = serde_json::from_value(json).map_err(|e| e.to_string())?;
    let config = ctx
        .config
        .and_then(|c| serde_json::from_value::<PluginConfig>(c).ok())
        .unwrap_or_default();
    let highlighted = highlight::highlight_codeblocks(&ctx.html, &config);
    ensure_theme_css(&config);
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

    if !html.contains("ts-") {
        return Ok(None);
    }

    let config: PluginConfig = ctx
        .config
        .as_ref()
        .map(|c| serde_json::from_value(c.clone()).unwrap_or_default())
        .unwrap_or_default();

    let css_href = config
        .css_path
        .unwrap_or_else(|| "/assets/tree-sitter-theme.css".to_string());

    let css_tag = format!(r#"<link rel="stylesheet" href="{css_href}" />"#);

    if let Some(pos) = html.find("</head>") {
        let mut result = String::with_capacity(html.len() + css_tag.len() + 1);
        result.push_str(&html[..pos]);
        result.push('\n');
        result.push_str(&css_tag);
        result.push('\n');
        result.push_str(&html[pos..]);
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

/// Write the default theme CSS to the site directory if it doesn't exist.
fn ensure_theme_css(config: &PluginConfig) {
    if config.css_path.is_some() {
        return;
    }
    let Ok(cwd) = std::env::current_dir() else {
        return;
    };

    let theme_path = cwd.join("assets").join("tree-sitter-theme.css");
    if theme_path.exists() {
        return;
    }

    let _ = std::fs::create_dir_all(cwd.join("assets"));

    if let Err(e) = std::fs::write(&theme_path, DEFAULT_THEME) {
        norgolith_plugin_sdk::plugin_log!("warn", "Failed to write theme CSS: {}", e);
    }
}

register_plugin!("norgolith-tree-sitter-highlight",
    hooks: [post_convert: post_convert_handler, post_render: post_render_handler]
);
