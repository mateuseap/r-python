# Standard Library (Metabuiltins)

This document describes the built-in functions (“metabuiltins”) implemented in Rust and exposed to RPython programs. These functions provide basic I/O, conversions, string/list helpers, and minimal tuple support.

Implementation lives in `src/stdlib/standard_library.rs` and is dispatched by the interpreter in `src/interpreter/expression_eval.rs`.

## Overview

Metabuiltins are special functions executed during interpretation. They receive an execution environment (where arguments are passed) and return a `Statement` (typically `Statement::Return(...)`).

The metabuiltin table is a global, lazily-initialized mapping:

```rust
pub type MetaBuiltinStmt = fn(&mut Environment<Expression>) -> Statement;

static METABUILTINS_TABLE: OnceLock<HashMap<String, MetaBuiltinStmt>>
```

## Available Functions

### I/O

- `input()` / `input(prompt)` → `String`
- `input_int()` / `input_int(prompt)` → `Int` (or an error string if parsing fails)
- `input_real()` / `input_real(prompt)` → `Real` (or an error string if parsing fails)
- `print(value)` → `Unit`
- `print_line(value)` → `Unit`

Notes:
- `print` does not add a newline.
- `print_line` adds a newline.

### Conversions & Formatting

- `to_string(value)` → `String`
- `to_string_fixed(value, places)` → `String`
- `to_int(value)` → `Int` (or an error string)
- `to_real(value)` → `Real` (or an error string)

### Strings & Collections

- `str_concat(left, right)` → `String`
- `join(values: List[String], sep)` → `String`
- `len(value)` → `Int` (supports `String`, `List`, and `Tuple`)

### Files

- `open(path, "r")` → `String`
- `open(path, "w", content)` → `Unit`
- `open(path, "a", content)` → `Unit`

Errors are returned as strings.

### Tuples

- `tuple_get(value, index)` → element value (or an error string)

`tuple_get` expects:
- `value`: a tuple value
- `index`: an integer index (0-based)

## Value Rendering

Runtime output for composite values is formatted as RPython literals (rather than leaking internal AST wrapper names):

- lists print like `[1, 2, 3]`
- tuples print like `(404, "Not Found")`
- `Maybe`/`Result` values print like `Just(1)`, `Nothing`, `Ok(999)`, `Err("boom")`

## Extending the Library

To add a new metabuiltin:

1. Implement a function with signature `MetaBuiltinStmt`.
2. Register it in `get_metabuiltins_table()`.
3. Add interpreter dispatch (if needed) for argument handling.
4. Add/update tests to keep the table key set consistent.