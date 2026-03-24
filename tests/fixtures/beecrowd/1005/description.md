# 1005 - Average 1

**Category:** Beginner

## Description

Read two floating point values A and B, corresponding to two student grades. Calculate the student's weighted average, considering that grade A has weight 3.5 and grade B has weight 7.5. Print the message "MEDIA" followed by the calculated average.

The formula is: MEDIA = (A × 3.5 + B × 7.5) / 11.0

## Input

Two floating point values with one decimal place.

## Output

Print the message "MEDIA = " followed by the average value with 5 decimal places.

## Example

| Input | Output |
|-------|--------|
| 5.0<br>7.1 | MEDIA = 6.43182 |

## RPython Features Used

- `input_real()` builtin for reading floating point numbers
- Real number arithmetic (`*`, `+`, `/`)
- `val` for immutable variable declaration
- `to_string_fixed()` for formatting with 5 decimal places
- `str_concat()` for string concatenation
