var value = input_int();
var _ = print_line(value);

val banknotes = [100, 50, 20, 10, 5, 2, 1];

for note in banknotes:
    val count = value / note;
    var _ = print(count);
    var _ = print(" nota(s) de R$ ");
    var _ = print(note);
    var _ = print_line(",00");

    value = value - (count * note);
end
