val a = input_int();
val b = input_int();
val c = input_int();
val d = input_int();

val is_even = (a / 2) * 2 == a;

if (b > c) and (d > a) and ((c + d) > (a + b)) and (c > 0) and (d > 0) and is_even:
    var _ = print_line("Valores aceitos");
else:
    var _ = print_line("Valores nao aceitos");
end
