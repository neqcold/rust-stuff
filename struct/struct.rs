#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn intersects(&self, other: &Rect) -> bool {
        (self.x >= other.x && self.x <= other.x + other.width 
            || other.x >= self.x && other.x <= self.x + self.width)
            && (self.y >= other.y && self.y <= other.y + other.height
                || other.y >= self.y && other.y <= self.y + self.height)
    }
}

fn main() {
    let r1 = Rect { x:0, y:0, width:10, height:10 };
    let r2 = Rect { x:5, y:5, width: 5, height: 5 };
    println!("{:?} intersects {:?}: {}", r1, r2, r1.intersects(&r2));
    println!("{:?} area is {}", r1, r1.area());
    println!("{:?} area is {}", r2, r2.area());
}

