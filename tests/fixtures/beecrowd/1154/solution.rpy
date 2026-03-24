def read_age() -> Maybe[Int]:
    val age = input_int();
    if age < 0:
        return Nothing;
    else:
        return Just(age);
    end;
end;

var sum = 0;
var count = 0;

var m = read_age();
while not isNothing(m):
    val age = unwrap(m);
    sum = sum + age;
    count = count + 1;
    m = read_age();
end;

val avg = to_real(sum) / to_real(count);
var _ = print_line(to_string_fixed(avg, 2));
