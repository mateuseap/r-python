# 1035 - Selection Test 1

**Category:** Beginner

## Description

Read four integer values A, B, C, and D. Then check if the following conditions are all true:

- B is greater than C
- D is greater than A
- The sum of C and D is greater than the sum of A and B
- C and D are positive values
- A is even

If all conditions are satisfied, print "Valores aceitos" (Values accepted). Otherwise, print "Valores nao aceitos" (Values not accepted).

## Input

Four integers A, B, C, and D.

## Output

Print the corresponding message based on the validation.

## Example

| Input | Output |
|-------|--------|
| 5<br>6<br>7<br>8 | Valores nao aceitos |
| 2<br>3<br>2<br>6 | Valores aceitos |

## RPython Features Used

- `input_int()` builtin for reading integers
- `val` for immutable variables
- Complex boolean expressions with `and`
- Comparison operators (`>`, `==`)
- If/else conditional
- Even number check using integer division: `(a / 2) * 2 == a`
