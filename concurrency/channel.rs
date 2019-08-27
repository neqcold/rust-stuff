use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;
use std::io::prelude::*;


fn main() {
    let (tx, rx) = mpsc::channel();
    println!("doing some complex calculations in the other thread while");
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send(1337).unwrap();
    });
    
    println!("smokin' weed in the main thread");
    println!("yeee bo");
    for _ in 0..50 {
        print!("i");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(2000 / 50));
    }
    let recieved = rx.recv().unwrap();
    println!("\nalready calculated! Here it is {}", recieved);
}
