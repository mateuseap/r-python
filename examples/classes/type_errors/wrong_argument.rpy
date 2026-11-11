class Counter:
    var n: Int = 0;

    def add(self: Counter, k: Int) -> Unit:
        self.n = self.n + k;
    end;
end;

val c = Counter();
c.add("dois")
