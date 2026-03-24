val w1 = input();
val w2 = input();
val w3 = input();

if w1 == "vertebrado":
    if w2 == "ave":
        if w3 == "carnivoro":
            var _ = print_line("aguia");
        else:
            var _ = print_line("pomba");
        end
    else:
        if w3 == "onivoro":
            var _ = print_line("homem");
        else:
            var _ = print_line("vaca");
        end
    end
else:
    if w2 == "inseto":
        if w3 == "hematofago":
            var _ = print_line("pulga");
        else:
            var _ = print_line("lagarta");
        end
    else:
        if w3 == "hematofago":
            var _ = print_line("sanguessuga");
        else:
            var _ = print_line("minhoca");
        end
    end
end
