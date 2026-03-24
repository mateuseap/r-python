# Beecrowd Test Suite - Feature Coverage

This document summarizes the RPython language features showcased across all Beecrowd problems.

## Problems Overview

| Problem | Name | Primary Features Showcased |
|---------|------|---------------------------|
| 1000 | Hello World | Basic output, `print_line()` |
| 1001 | Extremely Basic | `val`, arithmetic, `input_int()` |
| 1002 | Area of Circle | `input_real()`, `to_string_fixed()`, `str_concat()` |
| 1005 | Average 1 | Weighted arithmetic, floating point |
| 1010 | String Length | **`len()`** on strings |
| 1018 | Banknotes | Integer division, modulo, `var` |
| 1019 | Time Conversion | Time arithmetic, multiple assignments |
| 1028 | To Real | **`to_real()`** conversion |
| 1035 | Selection Test | **`and`** operator, equality |
| 1036 | Fuel Spent | **`or`** operator |
| 1037 | Interval | `and` with ranges |
| 1038 | Snack | **`True`/`False`** literals, `elif` chain |
| 1046 | Game Time | **`to_int()`** conversion |
| 1049 | Animal | Deep `if/elif/else` nesting, string comparison |
| 1050 | DDD | `def` functions, multi-branch `elif` |
| 1051 | Join Words | **`join()`** on lists |
| 1052 | Month | `elif` chain for mapping |
| 1059 | Even Numbers | `while` loop, counter |
| 1060 | Positive Numbers | **`continue`** statement |
| 1071 | Sum Odd Numbers | **`not`** operator, `while` loop |
| 1078 | Multiplication Table | `for` loop, list iteration |
| 1101 | Sequence Sum | **Tuples**, `for` over tuple |
| 1114 | Fixed Password | **`break`** statement, `while True` |
| 1153 | Factorial | **Recursion**, `def` functions |

## Feature Coverage Matrix

### Control Flow

| Feature | Problems Using It |
|---------|-------------------|
| `if/else` | 1035, 1036, 1037, 1038, 1049, 1050, 1052, 1071, 1114, 1153 |
| `elif` chain | 1035, 1037, 1038, 1049, 1050, 1052 |
| `while` loop | 1059, 1060, 1071, 1114 |
| `for` loop | 1078, 1101 |
| `break` | **1114** |
| `continue` | **1060** |

### Operators

| Feature | Problems Using It |
|---------|-------------------|
| Arithmetic (`+`, `-`, `*`, `/`) | 1001, 1002, 1005, 1018, 1019, 1059, 1071, 1078, 1153 |
| Integer division/modulo (`/`, `%`) | 1018, 1019 |
| Equality (`==`, `!=`) | 1035, 1049, 1052, 1114 |
| Comparison (`<`, `>`, `<=`, `>=`) | 1035, 1037, 1059, 1071 |
| `and` | **1035**, 1037 |
| `or` | **1036** |
| `not` | **1071** |

### Variables & Functions

| Feature | Problems Using It |
|---------|-------------------|
| `val` (immutable) | All problems |
| `var` (mutable) | 1018, 1019, 1038, 1059, 1060, 1071 |
| `def` (function) | 1050, 1153 |
| Recursion | **1153** |

### Data Types

| Feature | Problems Using It |
|---------|-------------------|
| Integers | All problems |
| Reals (floats) | 1002, 1005, 1028, 1036, 1037, 1038 |
| Strings | 1000, 1010, 1049, 1051, 1052 |
| Booleans | 1038 |
| Lists | 1051, 1078 |
| Tuples | **1101** |

### Boolean Literals

| Feature | Problems Using It |
|---------|-------------------|
| `True` | **1038** |
| `False` | **1038** |

### Standard Library Functions

| Function | Problems Using It |
|----------|-------------------|
| `print_line()` | All problems |
| `input_int()` | Most problems |
| `input_real()` | 1002, 1005, 1028, 1036, 1037 |
| `input()` (string) | 1010, 1049, 1051 |
| `to_string()` | 1018, 1019, 1046, 1059, 1060, 1071, 1078, 1101 |
| `to_string_fixed()` | 1002, 1005, 1028, 1038 |
| `str_concat()` | 1002, 1005, 1038, 1060 |
| `len()` | **1010** |
| `join()` | **1051** |
| `to_int()` | **1046** |
| `to_real()` | **1028**, 1038 |

## Features NOT Yet Showcased

The following features exist in RPython but are not yet covered by Beecrowd problems:

| Feature | Status |
|---------|--------|
| `lambda` expressions | Parser works, interpreter not fully implemented |
| ADTs (Algebraic Data Types) | Parser works, type checker not implemented |
| `Maybe` type | Not showcased |
| `Result` type | Not showcased |
| `assert` statements | Only in unit tests |
| List indexing | Not showcased |
| File I/O (`open()`) | Not showcased |

## Running Tests

```bash
# Run all Beecrowd tests
cargo test --test beecrowd

# Run a specific problem
cargo test problem_1001 --test beecrowd

# Run with verbose output
cargo test --test beecrowd -- --nocapture
```
