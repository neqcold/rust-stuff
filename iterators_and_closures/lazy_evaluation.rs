use std::env;

struct Cacher<T>
    where T: Fn(f64) -> f64 
{
    calculation: T,
    value: Option<f64>,
}

impl<T> Cacher<T> 
    where T: Fn(f64) -> f64 
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: f64) -> f64 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn main() {
    let mut lazy_sqrt = Cacher::new(|n| n.sqrt());
    let mut lazy_3_pow = Cacher::new(|n| 3f64.powf(n));
    let default_sqrt = |n: f64| n.sqrt();
    let default_3_pow = |n: f64| 3f64.powf(n);

    let n: f64 = env::args().nth(1).unwrap().parse().unwrap();

    if env::var("EVAL_LAZY").is_err() {
        for i in 0..100_000_000 {
            let r = default_sqrt(default_3_pow(n));
            if i % 10_000_000 == 0 {
                println!("{}", r);
            }
        }
    } else {
        for i in 0..100_000_000 {
            let r = lazy_sqrt.value(lazy_3_pow.value(n));
            if i % 10_000_000 == 0 {
                println!("{}", r);
            }
        }
    }
}

