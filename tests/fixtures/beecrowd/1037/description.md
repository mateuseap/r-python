# 1037 - Interval

**Category:** Beginner

## Description

Read a floating point number and print a message indicating in which of the following intervals the number belongs:

| Interval | Message |
|----------|---------|
| [0, 25] | Intervalo [0,25] |
| (25, 50] | Intervalo (25,50] |
| (50, 75] | Intervalo (50,75] |
| (75, 100] | Intervalo (75,100] |
| Outside | Fora de intervalo |

Where `[` means inclusive and `(` means exclusive.

## Input

A floating point number.

## Output

Print the corresponding interval message or "Fora de intervalo" if outside all intervals.

## Example

| Input | Output |
|-------|--------|
| 25.01 | Intervalo (25,50] |
| 25.00 | Intervalo [0,25] |
| -5.0 | Fora de intervalo |

## RPython Features Used

- `input_real()` builtin for reading floating point numbers
- Multiple `elif` conditions
- Comparison operators (`>=`, `<=`, `>`)
- Compound boolean expressions with `and`
- `print_line()` for output
