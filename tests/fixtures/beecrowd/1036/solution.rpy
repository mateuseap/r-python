val d = input_real();
val f = input_real();
val c = input_real();

if d <= 0.0 or f <= 0.0:
    var _ = print_line("Dados invalidos");
else:
    val range = f * c;
    if range >= d:
        var _ = print_line("Possivel");
    else:
        var _ = print_line("Impossivel");
    end;
end;
