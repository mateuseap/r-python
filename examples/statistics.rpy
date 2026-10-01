var _ = print_line("=== RPython Statistics Calculator ===");
var _ = print_line("Enter numbers one by one. Enter -1 to finish.");

var count = 0;
var total = 0;
var max_val = 0;
var min_val = 0;
var first = True;

var reading = True;
while reading:
    var _ = print("Enter a number (-1 to stop): ");
    val num = input_int();
    if num == -1:
        reading = False;
    else:
        count = count + 1;
        total = total + num;
        if first:
            max_val = num;
            min_val = num;
            first = False;
        else:
            if num > max_val:
                max_val = num;
            end;
            if num < min_val:
                min_val = num;
            end;
        end;
    end;
end;

var _ = print_line("");
var _ = print("Count: ");
var _ = print_line(count);

if count > 0:
    var _ = print("Sum: ");
    var _ = print_line(total);

    var _ = print("Average: ");
    var _ = print_line(total / count);

    var _ = print("Maximum: ");
    var _ = print_line(max_val);

    var _ = print("Minimum: ");
    var _ = print_line(min_val);
else:
    var _ = print_line("No numbers were entered.");
end;

asserttrue(count >= 0, "count must be non-negative");
var _ = print_line("=== Done ===");
