var _ = print_line("Hello, RPython!");

val name = "World";
var _ = print("Hello, ");
var _ = print_line(name);

val x = 10;
val y = 20;
val sum = x + y;

var _ = print("10 + 20 = ");
var _ = print_line(sum);

asserttrue(sum == 30, "sum should be 30");
var _ = print_line("Done!");
