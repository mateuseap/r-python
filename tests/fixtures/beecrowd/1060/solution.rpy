var count = 0;
var i = 0;

while i < 6:
    val n = input_int();
    i = i + 1;
    if n <= 0:
        continue;
    end;
    count = count + 1;
end;

var _ = print_line(str_concat(to_string(count), " valores positivos"));
