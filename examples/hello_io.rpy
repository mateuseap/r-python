var _ = print_line("=== RPython Demo ===");

var _ = print_line("=== 1. Variables ===");
val pi = 3.14159;
var counter = 1;
var _ = print("pi = ");
var _ = print_line(pi);
var _ = print("counter = ");
var _ = print_line(counter);

var _ = print_line("=== 2. Literals ===");
var _ = print("Int: ");
var _ = print_line(42);
var _ = print("Real: ");
var _ = print_line(3.14);
var _ = print("String: ");
var _ = print_line("hello");
var _ = print("Bool: ");
var _ = print_line(True);

var _ = print_line("=== 3. Arithmetic ===");
var _ = print("10 + 5 = ");
var _ = print_line(10 + 5);
var _ = print("10 - 3 = ");
var _ = print_line(10 - 3);
var _ = print("6 * 7 = ");
var _ = print_line(6 * 7);
var _ = print("20 / 4 = ");
var _ = print_line(20 / 4);

var _ = print_line("=== 4. Comparisons ===");
var _ = print("5 == 5: ");
var _ = print_line(5 == 5);
var _ = print("10 > 3: ");
var _ = print_line(10 > 3);
var _ = print("2 < 1: ");
var _ = print_line(2 < 1);
var _ = print("True and True: ");
var _ = print_line(True and True);
var _ = print("False or True: ");
var _ = print_line(False or True);
var _ = print("not True: ");
var _ = print_line(not True);
var _ = print("not False: ");
var _ = print_line(not False);

var _ = print_line("=== 5. Conditionals ===");
val score = 85;
var _ = print("Score 85 gets grade: ");
if score >= 90:
    var _ = print_line("A");
elif score >= 80:
    var _ = print_line("B");
elif score >= 70:
    var _ = print_line("C");
else:
    var _ = print_line("F");
end;

var _ = print_line("=== 6. While Loop ===");
var _ = print("Counting: ");
var i = 0;
while i < 5:
    var _ = print(i);
    var _ = print(" ");
    i = i + 1;
end;
var _ = print_line("");

var _ = print_line("=== 7. For Loop ===");
var _ = print("List items: ");
val items = [10, 20, 30];
for item in items:
    var _ = print(item);
    var _ = print(" ");
end;
var _ = print_line("");

var _ = print_line("=== 8. Break/Continue ===");
var _ = print("Break at 3: ");
var j = 0;
while j < 10:
    if j == 3:
        break;
    end;
    var _ = print(j);
    var _ = print(" ");
    j = j + 1;
end;
var _ = print_line("");

var _ = print("Skip evens: ");
for k in [0, 1, 2, 3, 4, 5]:
    if k == 0:
        continue;
    elif k == 2:
        continue;
    elif k == 4:
        continue;
    end;
    var _ = print(k);
    var _ = print(" ");
end;
var _ = print_line("");

var _ = print_line("=== 9. Functions ===");
def factorial(n: Int) -> Int:
    if n <= 1:
        return 1;
    else:
        return n * factorial(n - 1);
    end;
end;

def add(a: Int, b: Int) -> Int:
    return a + b;
end;

var _ = print("factorial(5) = ");
var _ = print_line(factorial(5));
var _ = print("add(17, 25) = ");
var _ = print_line(add(17, 25));

var _ = print_line("=== 10. len() ===");
var _ = print("len(hello) = ");
var _ = print_line(len("hello"));
var _ = print("len([1,2,3,4]) = ");
var _ = print_line(len([1, 2, 3, 4]));

var _ = print_line("=== 11. Conversions ===");
var _ = print("to_string(123) = ");
var _ = print_line(to_string(123));
var _ = print("to_int(456) = ");
var _ = print_line(to_int("456"));
var _ = print("to_real(42) = ");
var _ = print_line(to_real(42));

var _ = print_line("=== 12. Fixed-point formatting ===");
var _ = print("to_string_fixed(3.14159, 2) = ");
var _ = print_line(to_string_fixed(3.14159, 2));

var _ = print_line("=== 13. String operations ===");
var _ = print("str_concat: ");
var _ = print_line(str_concat("Hello, ", "World!"));
var _ = print("join([a,b,c], -) = ");
var _ = print_line(join(["a", "b", "c"], "-"));

var _ = print_line("=== 14. Assertions ===");
asserttrue(1 + 1 == 2, "1 + 1 should equal 2");
assertfalse(1 > 2, "1 should not be greater than 2");
asserteq(5 * 5, 25, "5 * 5 should equal 25");
assertneq(10, 20, "10 should not equal 20");
var _ = print_line("All assertions passed!");

var _ = print_line("=== 15. File I/O ===");
val test_file = "examples/rpython_test.txt";
var _ = open(test_file, "w", "Hello from RPython!");
var _ = print("Wrote to file. Reading back: ");
val contents = open(test_file, "r");
var _ = print_line(contents);
var _ = open(test_file, "a", "Appended line.");
val contents2 = open(test_file, "r");
var _ = print("After append: ");
var _ = print_line(contents2);

var _ = print_line("=== 16. Explicit Typing & Error Handling ===");
def print_typed_list(lst: List[Int]) -> Int:
    var _ = print("Typed List[Int]: ");
    var _ = print_line(lst);
    return 0;
end;

def print_typed_tuple(tup: Tuple[Int, String]) -> Int:
    var _ = print("Typed Tuple[Int, String]: ");
    var _ = print_line(tup);
    return 0;
end;

def higher_order(func: fn(Int) -> Int, value: Int) -> Int:
    return func(value);
end;

def duplicate(x: Int) -> Int:
    return x * 2;
end;

var _ = print_typed_list([100, 200, 300]);
var _ = print_typed_tuple((404, "Not Found"));

var _ = print("higher_order(duplicate, 21): ");
var _ = print_line(higher_order(duplicate, 21));

val maybe_val = Nothing;
var _ = print("isNothing(Nothing)? ");
var _ = print_line(isNothing(maybe_val));

val ok_val = Ok(999);
var _ = print("isError(Ok(999))? ");
var _ = print_line(isError(ok_val));
var _ = print("unwrap(Ok(999)): ");
var _ = print_line(unwrap(ok_val));

val err_val = Err("boom");
var _ = print("isError(Err(boom))? ");
var _ = print_line(isError(err_val));

var _ = print_line("=== Demo Complete ===");
