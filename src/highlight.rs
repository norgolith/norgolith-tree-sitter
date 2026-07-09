use crate::ts_highlight::{self, PluginConfig};

/// Highlight all `<pre><code class="lang-X">` blocks in the HTML fragment.
///
/// Returns the HTML with highlighted code blocks.
pub fn highlight_codeblocks(html: &str, config: &PluginConfig) -> String {
    let re = code_block_regex();
    let mut result = String::with_capacity(html.len());
    let mut last_end = 0;

    for caps in re.captures_iter(html) {
        let full_match = caps.get(0).unwrap();
        let lang_class = caps.get(1).unwrap().as_str();
        let code_content = caps.get(2).unwrap().as_str();

        let lang = lang_class
            .strip_prefix("language-")
            .or_else(|| lang_class.strip_prefix("lang-"))
            .unwrap_or(lang_class);

        // HTML-decode the content for tree-sitter
        let decoded = html_decode(code_content);

        // Highlight with tree-sitter
        let highlighted = ts_highlight::highlight(&decoded, lang, config);

        // Copy text before this match
        result.push_str(&html[last_end..full_match.start()]);

        // Write the highlighted block
        result.push_str(&format!(
            r#"<pre><code class="language-{}">{}</code></pre>"#,
            lang, highlighted
        ));

        last_end = full_match.end();
    }

    // Copy remaining text
    result.push_str(&html[last_end..]);
    result
}

/// Regex to match `<pre><code class="lang-X">content</code></pre>` blocks.
fn code_block_regex() -> regex::Regex {
    regex::Regex::new(
        r#"<pre><code class="([^"]+)">([\s\S]*?)</code></pre>"#,
    )
    .unwrap()
}

/// Decode common HTML entities in the code content.
///
/// Norg→HTML encodes `<`, `>`, `&` as HTML entities. We need to decode them
/// before passing to tree-sitter, then re-encode when producing output.
pub fn html_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '&' {
            let entity: String = chars.by_ref().take_while(|&ch| ch != ';').collect();
            match entity.as_str() {
                "lt" => result.push('<'),
                "gt" => result.push('>'),
                "amp" => result.push('&'),
                "quot" => result.push('"'),
                "apos" => result.push('\''),
                _ if entity.starts_with('#') => {
                    // Numeric character reference
                    if let Some(hex) = entity
                        .strip_prefix("#x")
                        .or_else(|| entity.strip_prefix("#X"))
                    {
                        if let Ok(code) = u32::from_str_radix(hex, 16) {
                            if let Some(ch) = char::from_u32(code) {
                                result.push(ch);
                            } else {
                                result.push_str(&format!("&{};", entity));
                            }
                        } else {
                            result.push_str(&format!("&{};", entity));
                        }
                    } else if let Ok(code) = entity.trim_start_matches('#').parse::<u32>() {
                        if let Some(ch) = char::from_u32(code) {
                            result.push(ch);
                        } else {
                            result.push_str(&format!("&{};", entity));
                        }
                    } else {
                        result.push_str(&format!("&{};", entity));
                    }
                }
                _ => {
                    result.push('&');
                    result.push_str(&entity);
                    result.push(';');
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_decode_entities() {
        assert_eq!(html_decode("fn foo() {}"), "fn foo() {}");
        assert_eq!(html_decode("&lt;div&gt;"), "<div>");
        assert_eq!(html_decode("a &amp; b"), "a & b");
        assert_eq!(html_decode("a &lt; b &gt; c"), "a < b > c");
    }

    #[test]
    fn test_code_block_regex() {
        let re = code_block_regex();
        let html = r#"<p>Before</p><pre><code class="language-rust">fn main() {}</code></pre><p>After</p>"#;
        let caps: Vec<_> = re.captures_iter(html).collect();
        assert_eq!(caps.len(), 1);
    }
}
