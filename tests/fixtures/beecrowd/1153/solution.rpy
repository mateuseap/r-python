def factorial(n: Int) -> Int:
    if n <= 1:
        return 1;
    else:
        return n * factorial(n - 1);
    end;
end;

val n = input_int();
var _ = print_line(factorial(n));
