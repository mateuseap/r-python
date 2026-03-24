# 1002 - Area of a Circle

**Category:** Beginner

## Description

The formula to calculate the area of a circle is: **area = π · r²**. Considering π = 3.14159, calculate the area given the radius.

## Input

The input contains a double-precision floating point value (real): the radius (R).

## Output

Print the message "A=" followed by the calculated area, with 4 decimal places.

## Example

| Input | Output |
|-------|--------|
| 2.00 | A=12.5664 |

## RPython Features Used

- `input_real()` builtin for reading floating point numbers
- Real number arithmetic (`*`)
- `val` for immutable variable declaration
- `to_string_fixed()` for formatting with fixed decimal places
- `str_concat()` for string concatenation
