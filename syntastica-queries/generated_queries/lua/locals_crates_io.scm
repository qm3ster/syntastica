[
  (chunk)
  (do_statement)
  (while_statement)
  (repeat_statement)
  (if_statement)
  (for_statement)
  (function_declaration)
  (function_definition)
] @local.scope

(assignment_statement
  (variable_list
    (identifier) @local.definition
  )
)

(assignment_statement
  (variable_list
    (dot_index_expression
      .
      (_) @local.definition
      (identifier) @local.definition
    )
  )
)

(
  (function_declaration
    name: (identifier) @local.definition
  )
  (#set! definition.function.scope "parent")
)

(
  (function_declaration
    name: (dot_index_expression
      .
      (_) @local.definition
      (identifier) @local.definition
    )
  )
  (#set! definition.method.scope "parent")
)

(
  (function_declaration
    name: (method_index_expression
      .
      (_) @local.definition
      (identifier) @local.definition
    )
  )
  (#set! definition.method.scope "parent")
)

(for_generic_clause
  (variable_list
    (identifier) @local.definition
  )
)

(for_numeric_clause
  name: (identifier) @local.definition
)

(parameters
  (identifier) @local.definition
)

[
  (identifier)
] @local.reference
