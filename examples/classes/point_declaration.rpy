class Point:
    var x: Int = 0;
    var y: Int = 0;

    def translate(self: Point, dx: Int, dy: Int) -> Unit:
        self.x = self.x + dx;
        self.y = self.y + dy;
    end;

    def get_x(self: Point) -> Int:
        return self.x;
    end;
end
