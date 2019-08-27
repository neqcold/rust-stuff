use std::fmt::Display;

struct Node<T: Display> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Display> Node<T> {
    fn new(val: T) -> Box<Node<T>> {
        Box::new(Node{ value: val, left: None, right: None })
    } 
    fn with_children(value: T, left: Box<Node<T>>, right: Box<Node<T>>) -> Box<Node<T>> {
        Box::new(Node{ value, left: Some(left), right: Some(right) })
    }
}

fn print_tree<T: Display>(node: &Box<Node<T>>, depth: u32) {
    for _ in 0..depth {
        print!("  ");
    }
    println!("{}", node.value);
    if let Some(left) = &node.left {
        print_tree(&left, depth + 1);
    }
    if let Some(right) = &node.right {
        print_tree(&right, depth + 1);
    }
}

fn main() {
    let root = Node::with_children("root",
                                   Node::with_children("left",
                                                       Node::new("0"),
                                                       Node::new("1")),
                                   Node::with_children("right",
                                                       Node::new("2"),
                                                       Node::new("3")),
                                  );
    print_tree(&root, 0);
}

