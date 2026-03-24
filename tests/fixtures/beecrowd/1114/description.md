# 1114 - Fixed Password

**Category:** Beginner (Repetition Structures)

## Description

Write a program that keeps reading passwords until the correct password (2002) is entered. For each incorrect password, print "Senha Invalida" (Invalid Password). When the correct password is entered, print "Acesso Permitido" (Access Granted) and stop.

## Input

Multiple integers, one per line, representing password attempts. The input ends when 2002 is read.

## Output

For each incorrect password, print "Senha Invalida". When the correct password is entered, print "Acesso Permitido".

## Example

| Input | Output |
|-------|--------|
| 2200<br>1020<br>2022<br>2002 | Senha Invalida<br>Senha Invalida<br>Senha Invalida<br>Acesso Permitido |

## RPython Features Used

- `input_int()` builtin for reading integers
- **`break` statement** to exit loop when password is correct
- `while True:` infinite loop pattern
- `if/else` conditional
- Integer equality comparison (`==`)
- `print_line()` for output
