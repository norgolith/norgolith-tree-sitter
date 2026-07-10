((content) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined))

((frontmatter
  (content) @injection.content)
  (#set! injection.language "yaml")
  (#set! injection.combined))
