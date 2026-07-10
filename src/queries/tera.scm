(identifier) @variable

((identifier) @variable.builtin
  (#any-of? @variable.builtin
    "loop"
    "__tera_context"))

(member_expression
  property: (identifier)? @variable.member)

(string) @string

(bool) @constant.builtin

(number) @number

[
  "."
  ","
] @punctuation.delimiter

[
  "*"
  "/"
  "%"
  "|"
  "+"
  "-"
  "~"
  "="
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
] @operator

[
  "("
  ")"
  "["
  "]"
  "{%"
  "%}"
  "-%}"
  "{%-"
  "}}"
  "{{"
  "-}}"
  "{{-"
  "::"
] @punctuation.bracket

(comment_tag) @comment

[
  "if"
  "elif"
  "else"
  "endif"
] @keyword.conditional

[
  "for"
  "endfor"
] @keyword.repeat

[
  "include"
  "import"
  "extends"
] @keyword.import

[
  "in"
  "and"
  "or"
  "not"
  "is"
] @keyword.operator

[
  "break"
  "continue"
] @keyword.return

[
  "set"
  "set_global"
  "filter"
  "endfilter"
  "block"
  "endblock"
  "macro"
  "endmacro"
  "raw"
  "endraw"
  "as"
] @keyword

(macro_statement
  name: (identifier) @function
  (parameter_list
    parameter: (identifier) @variable.parameter
    (optional_parameter
      name: (identifier) @variable.parameter)))

(call_expression
  scope: (identifier)? @namespace
  name: (identifier) @function)

(call_expression
  name: (identifier) @function.builtin
  (#any-of? @function.builtin
    "range"
    "now"
    "throw"
    "get_random"
    "get_env"))

(test_expression
  test: (identifier) @function)

(test_expression
  test: (identifier) @function.builtin
  (#any-of? @function.builtin
    "defined"
    "undefined"
    "odd"
    "even"
    "string"
    "number"
    "divisibleby"
    "iterable"
    "object"
    "starting_with"
    "ending_with"
    "containing"
    "matching"))

(filter_expression
  filter: (identifier) @function.method)

(filter_expression
  filter: (identifier) @function.builtin
  (#any-of? @function.builtin
    "lower"
    "upper"
    "wordcount"
    "capitalize"
    "replace"
    "addslashes"
    "slugify"
    "title"
    "trim"
    "trim_start"
    "trim_end"
    "trim_start_matches"
    "trim_end_matches"
    "truncate"
    "linebreaksbr"
    "spaceless"
    "indent"
    "striptags"
    "first"
    "last"
    "nth"
    "join"
    "length"
    "reverse"
    "sort"
    "unique"
    "slice"
    "group_by"
    "filter"
    "map"
    "concat"
    "urlencode"
    "urlencode_strict"
    "abs"
    "pluralize"
    "round"
    "filesizeformat"
    "date"
    "escape"
    "escape_xml"
    "safe"
    "get"
    "split"
    "int"
    "float"
    "json_encode"
    "as_str"
    "default"))

(import_statement
  scope: (identifier) @namespace)
