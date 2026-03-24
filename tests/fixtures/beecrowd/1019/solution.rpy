val n = input_int();

val hours = n / 3600;
var remaining = n - hours * 3600;

val minutes = remaining / 60;
remaining = remaining - minutes * 60;

val seconds = remaining;

var _ = print(hours);
var _ = print(":");
var _ = print(minutes);
var _ = print(":");
var _ = print_line(seconds);
