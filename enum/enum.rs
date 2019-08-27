struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Size {
    width: i32,
    height: i32,
}

impl Size {
    fn new(width: i32, height: i32) -> Size {
        Size { width, height }
    }
}

enum Shape {
    Line(Point, Point),
    Circle(Point, i32),
    Rectangle(Point, Size)
}

impl Shape {
    fn line(p1: Point, p2: Point) -> Shape {
        Shape::Line(p1, p2)
    }

    fn circle(pos: Point, radius: i32) -> Shape {
        Shape::Circle(pos, radius)
    }

    fn rectangle(pos: Point, size: Size) -> Shape {
        Shape::Rectangle(pos, size)
    }

    fn draw(&self) {
        match self {
            Shape::Line(p1, p2) => println!("Line {},{} {},{} has been drawn!", p1.x, p1.y, p2.x, p2.y),
            Shape::Circle(pos, radius) => println!("Circle {},{} with radius {} has been drawn!", pos.x, pos.y, radius),
            Shape::Rectangle(pos, size) => println!("Rectangle {},{} with size {}x{} has been drawn!", pos.x, pos.y, size.width, size.height),
        }
    }
}

fn main() {
    let line = Shape::line(Point::origin(), Point::new(10, 0));
    let circle = Shape::circle(Point::new(5, 5), 10);
    let rect = Shape::rectangle(Point::new(20, 30), Size::new(50, 20));
    let shapes = [line, circle, rect];

    for shape in shapes.iter() {
        shape.draw();
    }
}

