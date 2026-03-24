var _ = print_line("=== RPython Input Demo ===");
var _ = print_line("This demo showcases input(), input_int(), and input_real()");
var _ = print_line("");

var _ = print_line("--- input() ---");
val name = input("Enter your name: ");
var _ = print("Hello, ");
var _ = print(name);
var _ = print_line("!");

var _ = print_line("");
var _ = print_line("--- input_int() ---");
val age = input_int("Enter your age: ");
var _ = print("In 10 years you will be ");
var _ = print_line(age + 10);

var _ = print_line("");
var _ = print_line("--- input_real() ---");
val price = input_real("Enter a price: ");
val tax = price * 0.1;
var _ = print("Price with 10% tax: ");
var _ = print_line(to_string_fixed(price + tax, 2));

var _ = print_line("");
var _ = print_line("--- Combining inputs ---");
val a = input_int("Enter first number: ");
val b = input_int("Enter second number: ");
var _ = print(a);
var _ = print(" + ");
var _ = print(b);
var _ = print(" = ");
var _ = print_line(a + b);

var _ = print_line("");
var _ = print_line("=== Demo Complete ===");
