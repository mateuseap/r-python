# RPython 🚀

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![GitHub issues](https://img.shields.io/github/issues/UnBCIC-TP2/r-python)](https://github.com/UnBCIC-TP2/r-python/issues)
[![CI Status](https://img.shields.io/github/actions/workflow/status/UnBCIC-TP2/r-python/ci.yml?branch=main&label=ci-status&color=blue)](https://github.com/UnBCIC-TP2/r-python/actions)

RPython is a statically typed, interpreted language with Python-inspired syntax. Unlike Python, it uses explicit `end` delimiters instead of indentation and requires type annotations on function signatures. The entire toolchain—lexer, parser, type checker, interpreter, and standard library—is implemented in Rust.

> **Tip:** View `README.md` in *Raw* mode on GitHub to copy code snippets without HTML rendering.

---

## Table of Contents

1. [Motivation](#motivation)
2. [Language Overview](#language-overview)
3. [Type System](#type-system)
4. [Control Flow](#control-flow)
5. [Functions](#functions)
6. [Standard Library (Metabuiltins)](#standard-library-metabuiltins)
7. [Error Handling](#error-handling)
8. [Algebraic Data Types](#algebraic-data-types)
9. [Test Blocks](#test-blocks)
10. [Project Architecture](#project-architecture)
11. [Build & Test](#build--test)
12. [Running Programs](#running-programs)
13. [Current Limitations](#current-limitations)
14. [Contributing](#contributing)

---

## Motivation

RPython was created as a teaching tool for undergraduate courses on programming language implementation. The codebase demonstrates:

- **Parsing with combinators:** the parser uses `nom` to build an AST from source text.
- **Scoped environments:** lexical scoping is implemented via a stack of symbol tables.
- **Static type checking:** a type checker validates function signatures and expressions before execution.
- **Tree-walking interpretation:** the interpreter directly evaluates the AST.

The language surface resembles Python to lower the barrier for students, while the explicit block delimiters and mandatory type annotations expose concepts often hidden in dynamic languages.

---

## Language Overview

### Variables

```text
val pi = 3.14159;
var counter = 0;
counter = counter + 1;
```

- `val` declares an immutable variable; reassignment is a compile-time error.
- `var` declares a mutable variable.

> **Note:** RPython does not support comments in source code. The examples in this README omit comments for accuracy.

### Literals

| Type      | Example                        |
|-----------|--------------------------------|
| Integer   | `42`, `-7`                     |
| Real      | `3.14`, `0.0`                  |
| String    | `"hello"`, `"line\nbreak"`     |
| Boolean   | `True`, `False`                |
| List      | `[1, 2, 3]`                    |
| Tuple     | `(1, "a", True)`               |

### Operators

| Category     | Operators                              |
|--------------|----------------------------------------|
| Arithmetic   | `+`, `-`, `*`, `/`                     |
| Equality     | `==`, `!=`                             |
| Ordering     | `<`, `>`, `<=`, `>=`                   |
| Logical      | `and`, `or`, `not`                     |

> **Note:** Equality operators support numbers, strings, and booleans. Ordering operators are currently limited to numbers.

---

## Type System

RPython uses explicit, static types for function parameters and return values. Variables infer their type from the initializer expression.

### Built-in Types

- `Int` — 32-bit signed integer
- `Real` — 64-bit floating point
- `Boolean` — `True` or `False`
- `String` — UTF-8 text
- `Unit` — returned by side-effect-only functions (equivalent to `void`)
- `Any` — escape hatch; matches any type (used sparingly)

### Composite Types

- `List[T]` — homogeneous list of elements of type `T`
- `Tuple[T1, T2, ...]` — fixed-size, heterogeneous tuple
- `Maybe[T]` — optional value (`Just(value)` or `Nothing`)
- `Result[Ok, Err]` — success/failure carrier (`Ok(value)` or `Err(error)`)
- `fn(T1, T2) -> R` — first-class function type

---

## Control Flow

### Conditionals

```text
if score >= 90:
    grade = "A";
elif score >= 80:
    grade = "B";
else:
    grade = "C";
end
```

A single `end` closes the entire if-chain. The `elif` and `else` keywords introduce new branches without requiring separate `end` markers.

### Loops

```text
var i = 0;
while i < 5:
    var _ = print_line(to_string(i));
    i = i + 1;
end;

for x in [1, 2, 3]:
    var _ = print_line(to_string(x));
end;
```

- `break` exits the innermost loop immediately.
- `continue` skips to the next iteration.

---

## Functions

Functions require type annotations for parameters and return type.

```text
def factorial(n: Int) -> Int:
    if n <= 1:
        return 1;
    else:
        return n * factorial(n - 1);
    end;
end;

val result = factorial(5);
asserttrue(result == 120, "5! should be 120");
```

> **Syntax note:** Block statements (`if`, `while`, `for`, `def`) require a semicolon after the closing `end` when followed by additional statements at the same level.

### Lambdas

Anonymous functions can be assigned to variables or passed as arguments.

```text
def apply(f: fn(Int, Int) -> Int, a: Int, b: Int) -> Int:
    return f(a, b);
end;

val sum = apply(lambda (a: Int, b: Int) -> Int: return a + b end, 2, 3);
```

> **Current limitation:** Lambdas are supported as first-class function values (especially when passed as arguments to functions expecting a `fn(...) -> ...` type), but calling a lambda *via a variable name* is not fully supported yet in all contexts. Prefer `def` for named functions, or pass lambdas directly as arguments.

---

## Standard Library (Metabuiltins)

Metabuiltins are functions implemented in Rust and exposed to user code. They handle I/O, type conversions, and common operations.

### I/O

| Function                  | Description                                                    |
|---------------------------|----------------------------------------------------------------|
| `input()`                 | Read a line from stdin. Returns a `String`.                    |
| `input(prompt)`           | Print `prompt`, then read a line from stdin.                   |
| `input_int()`             | Read and parse an integer from stdin.                          |
| `input_int(prompt)`       | Print `prompt`, then read and parse an integer.                |
| `input_real()`            | Read and parse a real number from stdin.                       |
| `input_real(prompt)`      | Print `prompt`, then read and parse a real number.             |
| `print(value)`            | Print `value` without trailing newline.                        |
| `print_line(value)`       | Print `value` followed by a newline.                           |

### Conversion

| Function                         | Description                                       |
|----------------------------------|---------------------------------------------------|
| `to_string(value)`               | Convert any value to its string representation.   |
| `to_string_fixed(value, places)` | Format a number with fixed decimal places.        |
| `to_int(value)`                  | Convert a string or real to an integer.           |
| `to_real(value)`                 | Convert a string or integer to a real.            |

### Strings & Collections

| Function                          | Description                                                |
|-----------------------------------|------------------------------------------------------------|
| `str_concat(left, right)`         | Concatenate two strings.                                   |
| `join(values: List[String], sep)` | Join a list of strings with a separator.                   |
| `len(value)`                      | Return the length of a string, list, or tuple.             |

### Tuples

| Function                  | Description |
|--------------------------|-------------|
| `tuple_get(value, index)` | Return the element at `index` from a tuple (or an error string on invalid input). |

### Files

| Function                      | Description                                                       |
|-------------------------------|-------------------------------------------------------------------|
| `open(path, "r")`             | Read and return the contents of `path`.                           |
| `open(path, "w", content)`    | Write `content` to `path`, overwriting existing content.          |
| `open(path, "a", content)`    | Append `content` to `path`.                                       |

---

## Error Handling

RPython provides two monadic types for representing optional or fallible values: `Maybe[T]` and `Result[Ok, Err]`.

### Type Definitions

- `Maybe[T]` — Optional value: `Just(value)` or `Nothing`
- `Result[Ok, Err]` — Success/failure: `Ok(value)` or `Err(error)`

### Available Operations

- `isNothing(maybe)` — returns `True` if the value is `Nothing`
- `isError(result)` — returns `True` if the value is `Err`
- `unwrap(value)` — extracts the inner value (panics if `Nothing` or `Err`)
- `tryUnwrap(value)` — extracts or propagates errors automatically

> Note: `Just(value)`, `Nothing`, `Ok(value)`, and `Err(error)` are supported as expression syntax.

---

## Algebraic Data Types

ADTs can be declared with multiple constructors. Pattern matching is not yet implemented; values are constructed and passed around opaquely.

> **Current limitation:** ADT declarations are parsed as types but cannot yet be declared as top-level statements. The syntax shown below is the planned syntax; it is not yet functional.

```text
data Shape:
    | Circle Int
    | Rectangle Int Int
end

val c = Circle(5);
val r = Rectangle(3, 4);
```

---

## Test Blocks

Inline test definitions allow embedding unit tests directly in source files.

```text
test addition_works():
    val result = 2 + 2;
    asserttrue(result == 4, "2 + 2 should be 4");
end
```

Assertions:

| Function                          | Description                                 |
|-----------------------------------|---------------------------------------------|
| `assert(cond, msg)`               | Fail with `msg` if `cond` is false.         |
| `asserttrue(cond, msg)`           | Same as `assert`.                           |
| `assertfalse(cond, msg)`          | Fail if `cond` is true.                     |
| `asserteq(a, b, msg)`             | Fail if `a != b`.                           |
| `assertneq(a, b, msg)`            | Fail if `a == b`.                           |

---

## Project Architecture

```
src/
├── ir/              # AST definitions (expressions, statements, types)
├── parser/          # nom-based parsers for expressions, statements, types
├── type_checker/    # Static type checking for expressions and statements
├── interpreter/     # Tree-walking interpreter and test runner
├── stdlib/          # Metabuiltins table and implementations
├── pretty_print/    # AST → readable source formatter
├── environment/     # Scoped symbol tables (variables, functions, types)
└── main.rs          # Entry point (currently test-driven)
```

### Key Modules

- **`ir/ast.rs`** — Defines `Expression`, `Statement`, `Type`, `Function`, and ADT structures.
- **`parser/parser_stmt.rs`** — Parses statements (`if`, `while`, `for`, `def`, `break`, `continue`, etc.).
- **`parser/parser_expr.rs`** — Parses expressions (literals, operators, function calls).
- **`type_checker/`** — Validates types; checks function signatures and return types.
- **`interpreter/statement_execute.rs`** — Executes statements; handles loops with `break`/`continue`.
- **`interpreter/expression_eval.rs`** — Evaluates expressions; dispatches metabuiltin calls.
- **`stdlib/standard_library.rs`** — Implements the metabuiltins table and its built-in functions.
- **`pretty_print/`** — AST-to-source formatter (see below).

### Pretty Printer

The `pretty_print` module converts parsed AST nodes back into readable RPython source code. It is primarily used for:

- **Testing round-trip correctness:** parse → pretty-print → compare.
- **Debugging:** inspect how the parser understood your code.
- **Future tooling:** code formatters, IDE integrations.

The pretty printer is **not invoked during normal program execution**. It is available via the `prelude` module for programmatic use:

```rust
use r_python::prelude::{pretty, ToDoc};
// pretty(80, &statement.to_doc()) → formatted String
```

Optional feature flags enable performance instrumentation:

```bash
# Timing metrics
cargo run --features pp-timing --example pp_timing

# Profile counters
cargo run --features pp-profile --example pp_bench
```

---

## Build & Test

### Prerequisites

- Rust (stable toolchain)
- Cargo (bundled with Rust)

### Commands

```bash
# Clone the repository
git clone https://github.com/UnBCIC-TP2/r-python.git
cd r-python

# Build the project
cargo build

# Run all tests
cargo test

# Run tests with output (useful for debugging)
cargo test -- --nocapture
```

The test suite currently includes 270+ unit tests covering the parser, type checker, interpreter, and standard library.

---

## Running Programs

RPython includes a CLI to execute `.rpy` files directly:

```bash
# Run a program
cargo run -- path/to/program.rpy

# Or build first, then run the binary
cargo build --release
./target/release/r-python path/to/program.rpy
```

The interpreter reads from stdin and writes to stdout, making it suitable for automated judging systems like beecrowd.

---

## Current Limitations

1. **No module system:** all code lives in a single file.
2. **No pattern matching:** ADT constructors can be built but not destructured.
3. **No comments:** the parser does not support `#` or any comment syntax.
4. **No interactive REPL:** only file-based execution is supported.
5. **Limited error messages:** parser and type checker errors are functional but not always user-friendly.
6. **No tail-call optimization:** deep recursion may overflow the stack.
7. **Maybe/Result constructors:** supported as expression syntax, but values cannot be destructured without pattern matching.
8. **Small standard library:** only a small set of metabuiltins is available (basic I/O, conversions, simple string/list helpers); there are no rich libraries for math, dates/times, networking, etc.
9. **No exceptions:** there is no `try`/`catch` mechanism or exception hierarchy; errors are represented via `Maybe`/`Result` types or abort execution with an error message.
10. **No objects or methods:** there are no classes, interfaces, or method calls; programs are written with functions, lists/tuples, and algebraic data types.
11. **No concurrency or async:** the language has no built-in support for threads, async/await, or parallel execution.
12. **Interpreter-only, unoptimized:** execution is performed by a tree-walking interpreter without bytecode/JIT or optimization passes, so performance is below production-grade VMs/compilers.
13. **Minimal tooling:** beyond the CLI and internal pretty printer, there is no dedicated debugger, formatter binary, or IDE integration yet.

---

## Contributing

Contributions are welcome! Please read the contribution guides before submitting issues or pull requests:

- [Contributing Guidelines (English)](CONTRIBUTING_en.md)
- [Guia de Contribuição (Português)](CONTRIBUTING_pt.md)
