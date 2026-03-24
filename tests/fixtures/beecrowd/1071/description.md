# 1071 - Sum of Consecutive Odd Numbers I

**Category:** Beginner (Repetition Structures)

## Description

Read two integer values X and Y. Print the sum of all odd numbers between X and Y (exclusive).

## Input

Two integer values X and Y, not necessarily in order.

## Output

Print the sum of all odd integers strictly between X and Y.

## Example

| Input | Output |
|-------|--------|
| 6<br>-5 | 5 |
| 15<br>12 | 13 |
| 12<br>12 | 0 |

Note: In the first example, odd numbers strictly between -5 and 6 are: -3, -1, 1, 3, 5. Sum = 5.

## RPython Features Used

- `input_int()` builtin for reading integers
- `var` for mutable variables
- `while` loop for iteration
- Conditional logic with `if`
- Comparison and logical operators
- Integer arithmetic (`+`, `-`, `/`, `*`)
- Odd number check using: `(n / 2) * 2 != n`
