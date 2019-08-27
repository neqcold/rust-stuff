struct Id(i32);

enum Message {
    Hello { id: Id },
}

fn main() {
    let msg = Message::Hello { id: Id(10) };
    let accept_unknown_ids = true;

    match msg {
        Message::Hello { id: Id(x @ 1..=4) } => 
            println!("found an id in range [1;4]: {}", x),
        Message::Hello { id: Id(x @ 5..=7) } => 
            println!("found an id in range [5;7]: {}", x),
        Message::Hello { id: Id(x @ 8..=10) } => 
            println!("found an id in range [8;10]: {}", x),
        Message::Hello { id: Id(x) } if accept_unknown_ids => 
            println!("accepted a message the unknown id: {}", x),
        _ => println!("refused a message with an unknown id")
    }
}
