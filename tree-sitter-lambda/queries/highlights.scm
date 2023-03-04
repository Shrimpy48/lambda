(abstraction bound:(variable) @variable.parameter)
(function input_name:(variable) @variable.parameter)

(variable) @variable

(sort) @type.builtin

[
  (line_comment)
  (block_comment)
] @comment

[
  "\\"
  "λ"
  "Λ"
  "Π"
  "TT"
  "->"
  "→"
  "⟶"
] @operator

[
  "."
  ":"
  "::"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

