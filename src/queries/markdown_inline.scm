; From MDeiml/tree-sitter-markdown
(code_span) @markup.raw

(emphasis) @markup.italic

(strong_emphasis) @markup.strong

(strikethrough) @markup.strikethrough

(shortcut_link
  (link_text))

[
  (backslash_escape)
  (hard_line_break)
] @string.escape

; Conceal inline links
(inline_link
  [
    "["
    "]"
    "("
    (link_destination)
    ")"
  ] @markup.link)

[
  (link_label)
  (link_text)
  (link_title)
  (image_description)
] @markup.link.label

; Conceal image links
(image
  [
    "!"
    "["
    "]"
    "("
    (link_destination)
    ")"
  ] @markup.link)

; Conceal full reference links
(full_reference_link
  [
    "["
    "]"
    (link_label)
  ] @markup.link)

; Conceal collapsed reference links
(collapsed_reference_link
  [
    "["
    "]"
  ] @markup.link)

; Conceal shortcut links
(shortcut_link
  [
    "["
    "]"
  ] @markup.link)

[
  (link_destination)
  (uri_autolink)
  (email_autolink)
] @markup.link.url

(entity_reference)
