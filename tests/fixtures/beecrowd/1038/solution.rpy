val code = input_int();
val qty = input_int();

var valid = True;
var price = 0.0;

if code == 1:
    price = 4.0;
elif code == 2:
    price = 4.5;
elif code == 3:
    price = 5.0;
elif code == 4:
    price = 2.0;
elif code == 5:
    price = 1.5;
else:
    valid = False;
end;

if valid == True:
    val total = price * to_real(qty);
    var _ = print_line(str_concat("Total: R$ ", to_string_fixed(total, 2)));
else:
    var _ = print_line("Codigo invalido");
end;
