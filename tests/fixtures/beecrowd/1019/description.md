# 1019 - Time Conversion

**Category:** Beginner

## Description

Read an integer value representing a duration in seconds. Convert it to hours, minutes, and seconds (H:M:S format).

## Input

An integer N representing the time in seconds (0 ≤ N < 86400).

## Output

Print the time in the format "H:M:S" where H = hours, M = minutes, S = remaining seconds.

## Example

| Input | Output |
|-------|--------|
| 556 | 0:9:16 |
| 3600 | 1:0:0 |

## RPython Features Used

- `input_int()` builtin for reading integers
- Integer division (`/`) for extracting hours and minutes
- Manual modulo calculation: `n - (n / d) * d`
- `var` for mutable variables
- `print()` for formatted output with colons
