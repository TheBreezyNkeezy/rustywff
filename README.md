# RustyWFF

A propositional (and soon other) logic doodler inspired by Tsoding's Noq (https://github.com/tsoding/Noq).

## Todos

- Make EVERYTHING an S-expression for consistency
- Properly implement the quit command token
- Support for anonymous rules in application clauses
- Revamp token type to generic with specific kind and line location
- Support for free variables
- Shaping blocks with history
- Semantics for each expression
  - Propositional logic -- running truth tables or sequent calculus?
    - Setting of custom truth values for propositions (narrows the rows)
  - First order logic -- domain of discourse model
    - Requires support for custom predicates and functions (e.g. :bind predicate {arity} {name}
    - Requires setting domain of discourse
    - Same setting of custom truth values for constants and relations
- List all application result submatches
- Rule reverse and rule compose functions
