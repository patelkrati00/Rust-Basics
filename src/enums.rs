//when there are so many types use enums

enum Direction {
    North,
    East,
    West,
    South,
}

fn direction_info() {
    // We can only send one of the 4 directions
    // Nothing else is allowed (this makes code safe)
    move_around(Direction::North);
}

fn move_around(direction: Direction) {
    // logic
}

//Enums with Values

pub enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

pub fn cal_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius* radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
    }
}
