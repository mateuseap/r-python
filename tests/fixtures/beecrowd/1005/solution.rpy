val a = input_real();
val b = input_real();

val media = (a * 3.5 + b * 7.5) / 11.0;

var _ = print_line(str_concat("MEDIA = ", to_string_fixed(media, 5)));
