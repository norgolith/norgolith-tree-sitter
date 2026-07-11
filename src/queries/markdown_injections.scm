; Language injections for code blocks
((fenced_code_block
  (info_string
    (language) @injection.language)
  (code_fence_content) @injection.content)
  (#set! injection.include-children))

((html_block) @injection.content
  (#set! injection.language "html"))

(document
  .
  (section
    .
    (thematic_break)
    (_) @injection.content
    (thematic_break))
  (#set! injection.language "yaml"))

((minus_metadata) @injection.content
  (#set! injection.language "yaml"))

((plus_metadata) @injection.content
  (#set! injection.language "toml"))

; Inline content: include children so emphasis/code delimiters are visible
((inline) @injection.content
  (#set! injection.language "markdown_inline")
  (#set! injection.include-children))
