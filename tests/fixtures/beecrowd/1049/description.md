# 1049 - Animal

**Category:** Beginner

## Description

Write a program that reads three words and identifies an animal based on a decision tree:

1. First word: "vertebrado" (vertebrate) or "invertebrado" (invertebrate)
2. Second word: type of animal
3. Third word: characteristic

Based on the combination, identify the animal:

| Vertebrate | Type | Characteristic | Animal |
|------------|------|----------------|--------|
| vertebrado | ave | carnivoro | aguia |
| vertebrado | ave | onivoro | pomba |
| vertebrado | mamifero | onivoro | homem |
| vertebrado | mamifero | herbivoro | vaca |
| invertebrado | inseto | hematofago | pulga |
| invertebrado | inseto | herbivoro | lagarta |
| invertebrado | anelideo | hematofago | sanguessuga |
| invertebrado | anelideo | onivoro | minhoca |

## Input

Three words, one per line.

## Output

Print the name of the identified animal.

## Example

| Input | Output |
|-------|--------|
| vertebrado<br>ave<br>carnivoro | aguia |

## RPython Features Used

- `input()` builtin for reading strings
- String equality comparison (`==`)
- Deeply nested if/else conditionals
- `val` for immutable variables
- `print_line()` for output
