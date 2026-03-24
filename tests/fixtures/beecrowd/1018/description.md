# 1018 - Banknotes

**Category:** Beginner

## Description

Read an integer value N (less than 10⁶). Print the minimum number of banknotes needed to represent this value. Consider the banknotes of 100, 50, 20, 10, 5, 2, and 1 monetary units.

## Input

The input contains a positive integer N.

## Output

Print the minimum quantity of banknotes necessary to represent the input value, one banknote type per line, followed by the quantity.

## Example

| Input | Output |
|-------|--------|
| 576 | 576<br>5 nota(s) de R$ 100,00<br>1 nota(s) de R$ 50,00<br>1 nota(s) de R$ 20,00<br>0 nota(s) de R$ 10,00<br>1 nota(s) de R$ 5,00<br>0 nota(s) de R$ 2,00<br>1 nota(s) de R$ 1,00 |

## RPython Features Used

- `input_int()` builtin for reading integers
- `var` for mutable variable declaration
- Integer arithmetic (`/`, `*`, `-`)
- Manual modulo calculation (RPython uses integer division)
- `print()` and `print_line()` for formatted output
