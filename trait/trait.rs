#[derive(Debug)]
struct Color(u8, u8, u8);

trait Draw {
    fn draw(&self) {
        println!("drew nothin'");
    }
}

#[derive(Debug)]
struct Line {
    x: i32,
    y: i32,
    xx: i32,
    yy: i32,
    color: Color
}

impl Draw for Line {
    fn draw(&self) {
        println!("drew the line {:?}", self);
    }
}

#[derive(Debug)]
struct Circle {
    x: i32,
    y: i32,
    radius: i32,
    color: Color
}

impl Draw for Circle {
    fn draw(&self) {
        println!("drew the circle {:?}", self);
    }
}

impl Draw for Color {}

fn reset_scene() {
    println!("scene's been reset");
}

fn display_scene() {
    println!("scene's been displayed");
}

fn render_shit(shit: impl Draw){
    reset_scene();
    shit.draw();
    display_scene();
}

fn main() {
    let color = Color(0, 0, 0);
    let circle = Circle { x: 0, y: 0, radius: 10, color: Color(255, 255, 255) };
    let d1 = Line { x: 10, y: 0, xx: 10, yy: 20, color: Color(255, 0, 0) };
    let d2 = Line { x: 0, y: 10, xx: 20, yy: 10, color: Color(0, 255, 0) };

    circle.draw();
    d1.draw();
    d2.draw();
    color.draw();

    render_shit(circle);
}

