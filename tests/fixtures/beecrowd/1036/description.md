# Problem 1036 - Fuel Spent

**Beecrowd ID**: 1036 (adapted)
**Level**: Easy
**Category**: Sequential

## Description

A driver wants to know if they can reach their destination without refueling. Read the distance to travel (D km), the fuel in tank (F liters), and the car's consumption (C km/liter).

If the distance is zero or negative, or if the fuel is zero or negative, print "Dados invalidos". Otherwise, calculate if they can reach the destination.

## Features Showcased

- **`or` operator**: Check multiple invalid conditions
- **Floating point arithmetic**
- **Conditional logic**

## Input

Three floating point values: D (distance), F (fuel), C (consumption in km/liter).

## Output

"Dados invalidos" if D <= 0 or F <= 0, otherwise "Possivel" or "Impossivel".

## Example

**Input:**
```
100.0
10.0
12.0
```

**Output:**
```
Possivel
```
