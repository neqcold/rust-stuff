fn main() {
    let x = 5;

    match x {
        2..=4 => println!("[2; 4] contains x"),
        -1 | 1 => println!("x is equals either -1 or 1"),
        smthing_else => println!("x is smthing else: {}", smthing_else),
    }
}
