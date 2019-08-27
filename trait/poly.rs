trait Draw {
    fn draw(&self);
}

struct Scene {
    objects: Vec<Box<dyn Draw>>,
}

impl Scene {
    fn new() -> Scene {
        Scene { objects: vec![] }
    }

    fn add_object(mut self, object: Box<dyn Draw>) -> Self {
        self.objects.push(object);
        self
    }

    fn display(&self) {
        for obj in &self.objects {
            obj.draw();
        }
    }
}

struct Rect;
struct Circle;

impl Draw for Rect {
    fn draw(&self) {
        println!("drew a rect");
    }
}

impl Draw for Circle  {
    fn draw(&self) {
        println!("drew a circle");
    }
}

fn main() {
    Scene::new()
        .add_object(Box::new(Circle{}))
        .add_object(Box::new(Circle{}))
        .add_object(Box::new(Rect{}))
        .display();
}

