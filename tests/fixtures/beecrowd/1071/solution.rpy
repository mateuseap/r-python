val x = input_int();
val y = input_int();

var menor = x;
var maior = y;

if x > y:
    menor = y;
    maior = x;
end;

var soma = 0;
var i = menor + 1;

while i < maior:
    val is_even = (i / 2) * 2 == i;
    if not is_even:
        soma = soma + i;
    end;
    i = i + 1;
end;

var _ = print_line(soma);
