def calculate_area(r: Real) -> Real:
    val pi = 3.14159;
    return pi * r * r;
end;

val r = input_real();
val area = calculate_area(r);
var _ = print("A=");
var _ = print_line(to_string_fixed(area, 4));
