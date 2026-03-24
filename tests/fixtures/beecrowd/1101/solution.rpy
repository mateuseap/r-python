val a = input_int();
val b = input_int();
val c = input_int();

val values = (a, b, c);
var sum = 0;

for x in values:
    sum = sum + x;
end;

var _ = print_line(to_string(sum));
