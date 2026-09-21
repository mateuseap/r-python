class Point:
    var x: Int = 0;
    var y: Int = 0;

    def init(self: Point, x: Int, y: Int) -> Unit:
        self.x = x;
        self.y = y;
    end;

    def get_x(self: Point) -> Int:
        return self.x;
    end;
end;

val p = Point();
p.init(3, 4);
val px = p.get_x();
assert(px == 3, "x deveria ser 3");
