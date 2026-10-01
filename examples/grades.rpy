var _ = print_line("=== Student Grade Calculator ===");

def average(a: Real, b: Real, c: Real) -> Real:
    return (a + b + c) / 3.0;
end;

def status(score: Real) -> String:
    if score >= 70:
        return "Passed";
    else:
        return "Failed";
    end;
end;

val math = 85.5;
val science = 92.0;
val history = 78.0;

val avg = average(math, science, history);

var _ = print("Math: ");
var _ = print_line(math);
var _ = print("Science: ");
var _ = print_line(science);
var _ = print("History: ");
var _ = print_line(history);

var _ = print("Average: ");
var _ = print_line(avg);

var _ = print("Status: ");
var _ = print_line(status(avg));

asserttrue(average(90.0, 90.0, 90.0) == 90.0, "average of 90s should be 90");
asserttrue(status(80.0) == "Passed", "80 should pass");
asserttrue(status(60.0) == "Failed", "60 should fail");

var _ = print_line("=== Done ===");
