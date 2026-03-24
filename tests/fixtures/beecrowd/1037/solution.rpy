val x = input_real();

if x >= 0.0 and x <= 25.0:
    var _ = print_line("Intervalo [0,25]");
elif x > 25.0 and x <= 50.0:
    var _ = print_line("Intervalo (25,50]");
elif x > 50.0 and x <= 75.0:
    var _ = print_line("Intervalo (50,75]");
elif x > 75.0 and x <= 100.0:
    var _ = print_line("Intervalo (75,100]");
else:
    var _ = print_line("Fora de intervalo");
end
