var _ = print_line("=== RPython Math Demo ===");

def factorial(n: Int) -> Int:
    if n <= 1:
        return 1;
    else:
        return n * factorial(n - 1);
    end;
end;

def sum_one_to_n(n: Int) -> Int:
    var total = 0;
    var i = 1;
    while i <= n:
        total = total + i;
        i = i + 1;
    end;
    return total;
end;

def digit_sum(n: Int) -> Int:
    var x = n;
    if x < 0:
        x = x * -1;
    end;
    var total = 0;
    while x > 0:
        val last = x - (x / 10) * 10;
        total = total + last;
        x = x / 10;
    end;
    return total;
end;

var _ = print("Enter a positive integer: ");
val num = input_int();

var _ = print("Factorial of ");
var _ = print(num);
var _ = print(" = ");
var _ = print_line(factorial(num));

var _ = print("Sum of 1 to ");
var _ = print(num);
var _ = print(" = ");
var _ = print_line(sum_one_to_n(num));

var _ = print("Digit sum of ");
var _ = print(num);
var _ = print(" = ");
var _ = print_line(digit_sum(num));

var _ = print_line("");
var _ = print_line("Multiplication table:");
var i = 1;
while i <= 10:
    var _ = print(num);
    var _ = print(" x ");
    var _ = print(i);
    var _ = print(" = ");
    var _ = print_line(num * i);
    i = i + 1;
end;

asserttrue(factorial(5) == 120, "factorial(5) should be 120");
asserttrue(sum_one_to_n(10) == 55, "sum 1..10 should be 55");
asserttrue(digit_sum(123) == 6, "digit sum of 123 should be 6");
asserttrue(digit_sum(-45) == 9, "digit sum of -45 should be 9");
var _ = print_line("=== Done ===");
