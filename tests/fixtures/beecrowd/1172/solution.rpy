def print_output(record: Tuple[Int, Int]) -> Int:
    val i = tuple_get(record, 0);
    val value = tuple_get(record, 1);
    var _ = print(str_concat("X[", to_string(i)));
    var _ = print("] = ");
    var _ = print_line(to_string(value));
    return 0;
end;

def sanitize(doc: Int) -> Int:
    if doc <= 0:
        val r = Err("nonpositive");
        if isError(r):
            return 1;
        else:
            return unwrap(r);
        end;
    else:
        val r = Ok(doc);
        if isError(r):
            return 1;
        else:
            return unwrap(r);
        end;
    end;
end;

def for_each_indexed(data: List[Int], f: fn(Tuple[Int, Int]) -> Int) -> Int:
    var i = 0;
    for doc in data:
        val value = sanitize(doc);
        var _ = f((i, value));
        i = i + 1;
    end;
    return 0;
end;

val data = [
    input_int(), input_int(), input_int(), input_int(), input_int(),
    input_int(), input_int(), input_int(), input_int(), input_int()
];

var _ = for_each_indexed(data, print_output);
