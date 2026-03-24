val text_num = input();
val add_val = input_real();

val num = to_real(text_num);
val result = num + add_val;

var _ = print_line(to_string_fixed(result, 2));
