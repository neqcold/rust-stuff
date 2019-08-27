pub struct EvenNumber {
    n: i64,
}

impl EvenNumber {
    pub fn new(n: i64) -> EvenNumber {
        if n % 2 != 0 {
            panic!("the number must be even");
        }
        EvenNumber{ n }
    }

    pub fn value(&self) -> i64 {
        self.n
    }
}

pub struct MyBox {
    pub length: u32,
    pub width: u32,
    pub height: u32,
}

impl MyBox {
    pub fn can_hold(&self, other: &MyBox) -> bool {
        (self.length >= other.length && self.width >= other.width
            || self.length >= other.width && self.width >= other.length)
            && self.height >= other.height
    }

    pub fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }
}

